pub struct StateStorage {
    storage: std::collections::HashMap<String, Vec<u8>>,
}

impl StateStorage {
    pub fn new() -> Self {
        StateStorage {
            storage: std::collections::HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: Vec<u8>) {
        self.storage.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<&Vec<u8>> {
        self.storage.get(&key)
    }

    pub fn get_u32_or_zero(&self, key: String) -> u32 {
        u32::from_le_bytes(self.get_value_slice_or_zeros::<4>(key))
    }
    pub fn get_u8_or_zero(&self, key: String) -> u8 {
        u8::from_le_bytes(self.get_value_slice_or_zeros::<1>(key))
    }
    pub fn get_value_slice_or_zeros<const N: usize>(&self, key: String) -> [u8; N] {
        let bytes = self.get(key).cloned().unwrap_or(vec![0u8; N]);
        let mut out = [0u8; N];
        let copy_len = bytes.len().min(N);
        out[..copy_len].copy_from_slice(&bytes[..copy_len]);
        out
    }
}
