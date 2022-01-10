pub type EventQueue<T> = Vec<T>;
pub type TimeDelta = std::time::Duration;

pub struct Resource<T> {
    inner: T,
}

impl<T> Resource<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    pub fn replace(&mut self, inner: T) {
        self.inner = inner;
    }
}
