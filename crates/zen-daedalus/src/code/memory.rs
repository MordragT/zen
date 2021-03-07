pub struct Memory {
    ptr: *mut u8,
    len: usize,
    cap: usize,
}

impl Memory {
    pub fn new(memory: Vec<u8>) -> Self {
        let (ptr, len, cap) = memory.into_raw_parts();
        Self { ptr, len, cap }
    }
    pub fn get<T>(&self, offset: usize) -> Option<&T> {
        if offset > self.len {
            return None;
        }
        let ptr = unsafe { self.ptr.offset(offset as isize) };
        unsafe { std::mem::transmute::<*mut u8, *const T>(ptr).as_ref() }
    }
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
