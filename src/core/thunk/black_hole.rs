use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::sync::{Mutex, PoisonError};

use futures::prelude::*;
use futures::task::{AtomicWaker, Context};

use super::super::unsafe_ref::Ref;

#[derive(Debug)]
pub struct BlackHole(Mutex<Inner>);

impl BlackHole {
    pub fn new() -> Self {
        BlackHole(Mutex::new(Inner::Wait(AtomicWaker::new())))
    }

    pub fn release(&self) -> Result<(), BlackHoleError> {
        let mut inner = self.0.lock()?;

        match *inner {
            Inner::Released => return Err(BlackHoleError::new("black hole is released twice")),
            Inner::Wait(ref w) => w.wake(),
        }

        *inner = Inner::Released;

        Ok(())
    }
}

impl Future for BlackHole {
    type Error = BlackHoleError;
    type Item = ();

    fn poll(&mut self, c: &mut Context) -> Poll<Self::Item, Self::Error> {
        (&*self).poll(c)
    }
}

impl Future for Ref<BlackHole> {
    type Error = BlackHoleError;
    type Item = ();

    fn poll(&mut self, c: &mut Context) -> Poll<Self::Item, Self::Error> {
        (&**self: &BlackHole).poll(c)
    }
}

impl<'a> Future for &'a BlackHole {
    type Error = BlackHoleError;
    type Item = ();

    fn poll(&mut self, c: &mut Context) -> Poll<Self::Item, Self::Error> {
        match *self.0.lock()? {
            Inner::Released => Ok(Async::Ready(())),
            Inner::Wait(ref w) => {
                w.register(c.waker());
                Ok(Async::Pending)
            }
        }
    }
}

#[derive(Debug)]
enum Inner {
    Released,
    Wait(AtomicWaker),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlackHoleError(String);

impl BlackHoleError {
    fn new(s: &str) -> Self {
        BlackHoleError(s.to_string())
    }
}

impl Display for BlackHoleError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for BlackHoleError {
    fn description(&self) -> &str {
        &self.0
    }
}

impl<T> From<PoisonError<T>> for BlackHoleError {
    fn from(e: PoisonError<T>) -> Self {
        BlackHoleError::new(e.description())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::{channel, Sender};
    use std::sync::Arc;
    use std::thread::sleep;
    use std::time::Duration;

    use futures::executor::ThreadPool;
    use futures::prelude::*;

    use super::*;

    #[derive(Clone, Debug)]
    struct ArcBlackHole(Arc<BlackHole>);

    impl ArcBlackHole {
        fn new() -> Self {
            ArcBlackHole(Arc::new(BlackHole::new()))
        }

        fn release(&self) -> Result<(), BlackHoleError> {
            self.0.release()
        }
    }

    impl Future for ArcBlackHole {
        type Error = BlackHoleError;
        type Item = ();

        fn poll(&mut self, c: &mut Context) -> Poll<Self::Item, Self::Error> {
            (&*self.0).poll(c)
        }
    }

    #[test]
    fn black_hole_new() {
        BlackHole::new();
    }

    #[test]
    fn black_hole_release() {
        BlackHole::new().release().unwrap();
    }

    #[async(boxed, send)]
    fn send(s: Sender<i32>, b: ArcBlackHole) -> Result<(), Never> {
        s.send(1).unwrap();
        await!(b).unwrap();
        s.send(3).unwrap();
        Ok(())
    }

    #[async(boxed, send)]
    fn release(s: Sender<i32>, b: ArcBlackHole) -> Result<(), Never> {
        s.send(2).unwrap();
        b.release().unwrap();
        Ok(())
    }

    #[test]
    fn black_hole_wait() {
        let mut p = ThreadPool::new().unwrap();

        let b = ArcBlackHole::new();
        let (s, r) = channel();

        assert!(r.try_recv().is_err());

        p.spawn_pinned(send(s.clone(), b.clone())).unwrap();

        sleep(Duration::from_millis(100));
        assert_eq!(r.recv().unwrap(), 1);
        assert!(r.try_recv().is_err());

        p.spawn_pinned(release(s.clone(), b.clone())).unwrap();

        sleep(Duration::from_millis(100));
        assert_eq!(r.recv().unwrap(), 2);

        sleep(Duration::from_millis(100));
        assert_eq!(r.recv().unwrap(), 3);
        assert!(r.try_recv().is_err());
    }
}
