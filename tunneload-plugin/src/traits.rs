/// This Trait needs to be implemented for any Configuration type that
/// you want to use in the Plugin
pub trait Config: Sized {
    /// This function serializes the Configuration into a block of memory
    ///
    /// # Returns
    /// * A block of memory that represents the underlying Data
    fn serialize_data(&self) -> Vec<u8>;

    /// This function attempts to deserialize the given Block of memory back
    /// into the Configuration for the Plugin
    ///
    /// # Params:
    /// * A Ptr to the beginning of the block of memory
    /// * The Size of the block of memory
    fn deserialize_data(addr: *mut u8, size: usize) -> Option<Self>;

    /// Returns the Length of the serialized Data
    fn len(&self) -> usize;
}

impl Config for String {
    fn serialize_data(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    fn deserialize_data(addr: *mut u8, size: usize) -> Option<Self> {
        let content = unsafe { String::from_raw_parts(addr, size, size) };

        Some(content)
    }

    fn len(&self) -> usize {
        String::len(self)
    }
}

impl Config for Vec<u8> {
    fn serialize_data(&self) -> Vec<u8> {
        self.clone()
    }

    fn deserialize_data(addr: *mut u8, size: usize) -> Option<Self> {
        unsafe { Some(Vec::from_raw_parts(addr, size, size)) }
    }

    fn len(&self) -> usize {
        Vec::len(self)
    }
}
