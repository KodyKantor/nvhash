use wasm_bindgen::prelude::*;
use sha2::{Digest};

#[wasm_bindgen]
pub fn hash(data: &[u8], out: &mut [u8]) {
    let mut hasher = sha2::Sha256::new();
    hasher.update(data);
    let dgst = hasher.finalize();
    out.copy_from_slice(&dgst);

}

#[wasm_bindgen]
pub fn create_hasher() -> Sha2Hash {
    Sha2Hash {
        hasher: sha2::Sha256::new(),
    }
}

#[wasm_bindgen]
pub fn create_keyed(_key_slice: &[u8]) -> Sha2Hash {
    unimplemented!();
}

#[wasm_bindgen]
pub fn create_derive(_context: String) -> Sha2Hash {
    unimplemented!();
}

#[wasm_bindgen]
pub struct Sha2Hash {
    hasher: sha2::Sha256,
}

#[wasm_bindgen]
impl Sha2Hash {
    pub fn reader(&mut self) -> HashReader {
        HashReader {}
    }

    pub fn update(&mut self, input_bytes: &[u8]) {
        self.hasher.update(input_bytes);
    }

    pub fn digest(&mut self, out: &mut [u8]) {
        let dgst = self.hasher.clone().finalize();
        out.copy_from_slice(&dgst);
    }
}

#[wasm_bindgen]
pub struct HashReader {}

#[wasm_bindgen]
impl HashReader {
    pub fn fill(&mut self, _bytes: &mut [u8]) {
        unimplemented!();
    }

    pub fn set_position(&mut self, _position: u64) {
        unimplemented!();
    }
}