use std::ptr::NonNull;

#[derive(Debug)]
pub struct Sendable<T>(NonNull<T>);

impl<'a, T> Sendable<T> {
    pub fn new(reference: &'a T) -> Sendable<T> {
        Sendable(NonNull::from(reference))
    }
}

unsafe impl<T> Send for Sendable<T> {}

impl<T> AsRef<T> for Sendable<T> {
    fn as_ref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}

impl<T> AsMut<T> for Sendable<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_ref() {
        let data = 42;
        let ptr = Sendable::new(&data);
        assert_eq!(*ptr.as_ref(), 42);
    }

    #[test]
    fn as_mut() {
        let data = 0;
        let mut ptr = Sendable::new(&data);
        *ptr.as_mut() = 42;
        assert_eq!(data, 42);
    }
}
