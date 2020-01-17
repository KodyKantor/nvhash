use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hash(data: &[u8], out: &mut [u8]) {
    let mut hasher = blake3::Hasher::new();
    hasher.update(data);
    let mut reader = hasher.finalize_xof();
    reader.fill(out);
}

#[wasm_bindgen]
pub struct Blake3Hash {
    hasher: blake3::Hasher,
}

#[wasm_bindgen]
impl Blake3Hash {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Blake3Hash {
        Blake3Hash {
            hasher: blake3::Hasher::new(),
        }
    }

    pub fn update(&mut self, input_bytes: &[u8]) {
        self.hasher.update(input_bytes);
    }

    pub fn digest(&mut self, out: &mut [u8]) {
        let mut reader = self.hasher.finalize_xof();
        reader.fill(out);
    }
}
