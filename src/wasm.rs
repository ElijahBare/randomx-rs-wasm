use wasm_bindgen::prelude::*;
use js_sys::{Uint8Array};

use crate::{RandomXCache, RandomXDataset, RandomXFlag, RandomXVM};

#[wasm_bindgen(start)]
pub fn start() {
    // Initialize panic hook for better error messages
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn init_logging() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct WasmRandomXCache {
    cache: RandomXCache,
}

#[wasm_bindgen]
impl WasmRandomXCache {
    #[wasm_bindgen(constructor)]
    pub fn new(flags: u32, key: &[u8]) -> Result<WasmRandomXCache, JsValue> {
        let rx_flags = RandomXFlag::from_bits(flags).ok_or_else(|| 
            JsValue::from_str("Invalid RandomX flags"))?;
        
        let cache = RandomXCache::new(rx_flags, key)
            .map_err(|e| JsValue::from_str(&format!("Failed to create RandomX cache: {}", e)))?;
        
        Ok(WasmRandomXCache { cache })
    }

    #[wasm_bindgen(js_name = getRecommendedFlags)]
    pub fn get_recommended_flags() -> u32 {
        RandomXFlag::get_recommended_flags().bits()
    }
}

#[wasm_bindgen]
pub struct WasmRandomXDataset {
    dataset: RandomXDataset,
}

#[wasm_bindgen]
impl WasmRandomXDataset {
    #[wasm_bindgen(constructor)]
    pub fn new(flags: u32, cache: WasmRandomXCache, start: u32) -> Result<WasmRandomXDataset, JsValue> {
        let rx_flags = RandomXFlag::from_bits(flags).ok_or_else(|| 
            JsValue::from_str("Invalid RandomX flags"))?;
        
        let dataset = RandomXDataset::new(rx_flags, cache.cache.clone(), start)
            .map_err(|e| JsValue::from_str(&format!("Failed to create RandomX dataset: {}", e)))?;
        
        Ok(WasmRandomXDataset { dataset })
    }

    #[wasm_bindgen(js_name = getCount)]
    pub fn get_count() -> Result<u32, JsValue> {
        RandomXDataset::count()
            .map_err(|e| JsValue::from_str(&format!("Failed to get dataset count: {}", e)))
    }
}

#[wasm_bindgen]
pub struct WasmRandomXVM {
    vm: RandomXVM,
}

#[wasm_bindgen]
impl WasmRandomXVM {
    #[wasm_bindgen(constructor)]
    pub fn new(
        flags: u32, 
        cache: Option<WasmRandomXCache>,
        dataset: Option<WasmRandomXDataset>
    ) -> Result<WasmRandomXVM, JsValue> {
        let rx_flags = RandomXFlag::from_bits(flags).ok_or_else(|| 
            JsValue::from_str("Invalid RandomX flags"))?;
        
        let cache_option = cache.map(|c| c.cache.clone());
        let dataset_option = dataset.map(|d| d.dataset.clone());
        
        let vm = RandomXVM::new(rx_flags, cache_option, dataset_option)
            .map_err(|e| JsValue::from_str(&format!("Failed to create RandomX VM: {}", e)))?;
        
        Ok(WasmRandomXVM { vm })
    }

    #[wasm_bindgen(js_name = calculateHash)]
    pub fn calculate_hash(&self, input: &[u8]) -> Result<Uint8Array, JsValue> {
        let hash_vec = self.vm.calculate_hash(input)
            .map_err(|e| JsValue::from_str(&format!("Failed to calculate hash: {}", e)))?;
        
        let result = Uint8Array::new_with_length(hash_vec.len() as u32);
        result.copy_from(&hash_vec);
        Ok(result)
    }

    #[wasm_bindgen(js_name = calculateHashSet)]
    pub fn calculate_hash_set(&self, inputs: Vec<js_sys::Uint8Array>) -> Result<Vec<Uint8Array>, JsValue> {
        // Convert Vec<Uint8Array> to Vec<&[u8]>
        let mut input_bytes: Vec<Vec<u8>> = Vec::with_capacity(inputs.len());
        let mut input_refs: Vec<&[u8]> = Vec::with_capacity(inputs.len());
        
        for input in inputs {
            let mut bytes = vec![0u8; input.length() as usize];
            input.copy_to(&mut bytes);
            input_bytes.push(bytes);
        }
        
        for bytes in &input_bytes {
            input_refs.push(bytes.as_slice());
        }
        
        let hash_vecs = self.vm.calculate_hash_set(&input_refs)
            .map_err(|e| JsValue::from_str(&format!("Failed to calculate hash set: {}", e)))?;
        
        // Convert Vec<Vec<u8>> to Vec<Uint8Array>
        let result: Vec<Uint8Array> = hash_vecs.into_iter().map(|hash_vec| {
            let arr = Uint8Array::new_with_length(hash_vec.len() as u32);
            arr.copy_from(&hash_vec);
            arr
        }).collect();
        
        Ok(result)
    }
}

#[wasm_bindgen]
pub fn get_flag_default() -> u32 { RandomXFlag::FLAG_DEFAULT.bits() }

#[wasm_bindgen]
pub fn get_flag_large_pages() -> u32 { RandomXFlag::FLAG_LARGE_PAGES.bits() }

#[wasm_bindgen]
pub fn get_flag_hard_aes() -> u32 { RandomXFlag::FLAG_HARD_AES.bits() }

#[wasm_bindgen]
pub fn get_flag_full_mem() -> u32 { RandomXFlag::FLAG_FULL_MEM.bits() }

#[wasm_bindgen]
pub fn get_flag_jit() -> u32 { RandomXFlag::FLAG_JIT.bits() }

#[wasm_bindgen]
pub fn get_flag_secure() -> u32 { RandomXFlag::FLAG_SECURE.bits() }

#[wasm_bindgen]
pub fn get_flag_argon2_ssse3() -> u32 { RandomXFlag::FLAG_ARGON2_SSSE3.bits() }

#[wasm_bindgen]
pub fn get_flag_argon2_avx2() -> u32 { RandomXFlag::FLAG_ARGON2_AVX2.bits() }

#[wasm_bindgen]
pub fn get_flag_argon2() -> u32 { RandomXFlag::FLAG_ARGON2.bits() }