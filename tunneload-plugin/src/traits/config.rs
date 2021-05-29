use std::convert::TryInto;

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
    /// * The block of memory
    fn deserialize_data(data: &[u8]) -> Option<Self>;

    /// Returns the Length of the serialized Data in bytes
    fn len(&self) -> usize;
}

impl Config for String {
    fn serialize_data(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    fn deserialize_data(data: &[u8]) -> Option<Self> {
        let content = match String::from_utf8(data.to_vec()) {
            Ok(c) => c,
            Err(_) => return None,
        };

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

    fn deserialize_data(data: &[u8]) -> Option<Self> {
        Some(data.to_vec())
    }

    fn len(&self) -> usize {
        Vec::len(self)
    }
}

impl Config for u16 {
    fn serialize_data(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn deserialize_data(data: &[u8]) -> Option<Self> {
        if data.len() != 2 {
            return None;
        }

        let data: [u8; 2] = data.try_into().unwrap();
        Some(Self::from_be_bytes(data))
    }

    fn len(&self) -> usize {
        2
    }
}
impl Config for u32 {
    fn serialize_data(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn deserialize_data(data: &[u8]) -> Option<Self> {
        if data.len() != 4 {
            return None;
        }

        let data: [u8; 4] = data.try_into().unwrap();
        Some(Self::from_be_bytes(data))
    }

    fn len(&self) -> usize {
        4
    }
}
impl Config for u64 {
    fn serialize_data(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn deserialize_data(data: &[u8]) -> Option<Self> {
        if data.len() != 8 {
            return None;
        }

        let data: [u8; 8] = data.try_into().unwrap();
        Some(Self::from_be_bytes(data))
    }

    fn len(&self) -> usize {
        8
    }
}
