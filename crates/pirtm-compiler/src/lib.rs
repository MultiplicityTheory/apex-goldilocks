use wasm_bindgen::prelude::*;

pub mod types;
pub mod ace;
pub mod linker;
pub mod validator;
pub mod diagnostic;
pub mod compiler;

pub use linker::{verify_module_manifest, ModuleAttributes};
pub use ace::{DynamicOperator, FixedPoint, SCALE_BASE};
pub use types::Sig;

#[wasm_bindgen]
pub fn validate_source(source: &str, prime_set_json: &str, stratum_id: &str) -> Result<JsValue, JsValue> {
    let envelope = compiler::validate_source(source, prime_set_json, stratum_id)
        .map_err(|e| JsValue::from_str(&e))?;
    
    Ok(serde_wasm_bindgen::to_value(&envelope).map_err(|e| JsValue::from_str(&e.to_string()))?)
}
