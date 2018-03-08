use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::sync::{Mutex, PoisonError};

use futures::prelude::*;
use futures::task::{self, Task};

use self::Inner::*;

#[derive(Debug)]
pub struct BlackHole(Mutex<Inner>);

impl BlackHole {
    pub fn new() -> Self {
        BlackHole(Mutex::new(Wait(vec![])))
    }

    pub fn release(&self) -> Result<(), BlackHoleError> {
        let mut inner = self.0.lock()?;

        match *inner {
            Released => return Err(BlackHoleError::new("black hole is released twice")),
            Wait(ref tasks) => for task in tasks {
                task.notify();
            },
        }

        *inner = Released;

        Ok(())
    }
}

impl Future for BlackHole {
    type Item = ();
    type Error = BlackHoleError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        (&*self).poll()
    }
}

impl<'a> Future for &'a BlackHole {
    type Item = ();
    type Error = BlackHoleError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match *self.0.lock()? {
            Released => Ok(Async::Ready(())),
            Wait(ref mut tasks) => {
                tasks.push(task::current());
                Ok(Async::NotReady)
            }
        }
    }
}

#[derive(Debug)]
enum Inner {
    Released,
    Wait(Vec<Task>),
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
    use std::sync::Arc;
    use std::sync::mpsc::{channel, Sender};
    use std::thread::sleep;
    use std::time::Duration;

    use futures::prelude::*;
    use futures_cpupool::CpuPool;

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
        type Item = ();
        type Error = BlackHoleError;

        fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
            (&*self.0).poll()
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

    #[async]
    fn send(s: Sender<i32>, b: ArcBlackHole) -> Result<(), BlackHoleError> {
        s.send(1).unwrap();
        await!(b)?;
        s.send(3).unwrap();
        Ok(())
    }

    #[async]
    fn release(s: Sender<i32>, b: ArcBlackHole) -> Result<(), BlackHoleError> {
        s.send(2).unwrap();
        b.release()?;
        Ok(())
    }

    #[test]
    fn black_hole_wait() {
        let p = CpuPool::new_num_cpus();

        let b = ArcBlackHole::new();
        let (s, r) = channel();

        assert!(r.try_recv().is_err());

        let f1 = p.spawn(send(s.clone(), b.clone()));

        sleep(Duration::from_millis(100));
        assert_eq!(r.recv().unwrap(), 1);
        assert!(r.try_recv().is_err());

        let f2 = p.spawn(release(s.clone(), b.clone()));

        sleep(Duration::from_millis(100));
        assert_eq!(r.recv().unwrap(), 2);

        sleep(Duration::from_millis(100));
        assert_eq!(r.recv().unwrap(), 3);
        assert!(r.try_recv().is_err());

        assert!(f1.wait().is_ok());
        assert!(f2.wait().is_ok());
    }
}
