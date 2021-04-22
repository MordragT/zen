/// Holds the Memory for the [Code](crate::code::Code)
pub struct Memory {
    ptr: *mut u8,
    len: usize,
    _cap: usize,
}

impl Memory {
    /// Creates a new memory object from a vector containing bytes
    pub fn new(memory: Vec<u8>) -> Self {
        let (ptr, len, _cap) = memory.into_raw_parts();
        Self { ptr, len, _cap }
    }
    /// Returns an immutable reference to the specified value type at the given offset
    pub fn get<T>(&self, offset: usize) -> Option<&T> {
        if offset > self.len {
            return None;
        }
        let ptr = unsafe { self.ptr.offset(offset as isize) };
        unsafe { std::mem::transmute::<*mut u8, *const T>(ptr).as_ref() }
    }
    /// Returns a mutable reference to the specified value type at the given offset
    pub fn get_mut<T>(&mut self, offset: usize) -> Option<&mut T> {
        if offset > self.len {
            return None;
        }
        let ptr = unsafe { self.ptr.offset(offset as isize) };
        unsafe { std::mem::transmute::<*mut u8, *mut T>(ptr).as_mut() }
    }
}

impl From<Vec<u8>> for Memory {
    fn from(memory: Vec<u8>) -> Self {
        Self::new(memory)
    }
}
