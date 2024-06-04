use wasm_bindgen::prelude::*;
use base64::prelude::*;

#[wasm_bindgen]
pub fn encode3(input: &str) -> String {
   BASE64_STANDARD.encode(input)
}

#[wasm_bindgen]
pub fn decode3(input: &str) -> Result<String, JsValue> {
    match BASE64_STANDARD.decode(input) {
        Ok(s) => {
            match String::from_utf8(s) {
                Ok(s) => Ok(s),
                Err(e) => Err(JsValue::from_str(&format!("{}", e))),
            }
        },
        Err(e) => Err(JsValue::from_str(&format!("{}", e))),
    }
}


#[wasm_bindgen]
pub fn encode4(input: &str) -> String {
    fast32::base64::RFC4648.encode(input.as_ref())
}

#[wasm_bindgen]
pub fn decode4(input: &str) -> Result<String, JsValue> {
    match fast32::base64::RFC4648.decode(input.as_ref()) {
        Ok(s) => {
            match String::from_utf8(s) {
                Ok(s) => Ok(s),
                Err(e) => Err(JsValue::from_str(&format!("{}", e))),
            }
        },
        Err(e) => Err(JsValue::from_str(&format!("{}", e))),
    }
}
