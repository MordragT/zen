/// Holds the Memory for the [Code](crate::code::Code)
pub struct Memory {
    raw: Vec<u8>,
}

impl Memory {
    /// Creates a new memory object from a vector containing bytes
    pub fn new(raw: Vec<u8>) -> Self {
        Self { raw }
    }
    /// Returns an immutable reference to the specified value type at the given offset
    pub fn get<T>(&self, offset: usize) -> Option<&T> {
        let ptr = self.raw.get(offset)? as *const u8;
        unsafe { std::mem::transmute::<*const u8, *const T>(ptr).as_ref() }
    }
    /// Returns a mutable reference to the specified value type at the given offset
    pub fn get_mut<T>(&mut self, offset: usize) -> Option<&mut T> {
        let ptr = self.raw.get_mut(offset)? as *mut u8;
        unsafe { std::mem::transmute::<*mut u8, *mut T>(ptr).as_mut() }
    }
}

impl From<Vec<u8>> for Memory {
    fn from(memory: Vec<u8>) -> Self {
        Self::new(memory)
    }
}
