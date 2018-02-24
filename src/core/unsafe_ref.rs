use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug)]
pub struct Ref<T>(pub *const T);

impl<T> Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref().unwrap() }
    }
}

impl<'a, T> From<&'a T> for Ref<T> {
    fn from(x: &'a T) -> Self {
        Ref(x)
    }
}

unsafe impl<T> Send for Ref<T> {}
unsafe impl<T> Sync for Ref<T> {}

#[derive(Clone, Debug)]
pub struct RefMut<T>(pub *mut T);

impl<T> Deref for RefMut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref().unwrap() }
    }
}

impl<T> DerefMut for RefMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut().unwrap() }
    }
}

impl<'a, T> From<&'a mut T> for RefMut<T> {
    fn from(x: &'a mut T) -> Self {
        RefMut(x)
    }
}

unsafe impl<T> Send for RefMut<T> {}
unsafe impl<T> Sync for RefMut<T> {}
