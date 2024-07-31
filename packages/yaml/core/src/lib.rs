use serde_json::Value;
use serde_wasm_bindgen::{from_value, to_value};
use serde_yaml;
use wasm_bindgen::prelude::*;

// 从 YAML 字符串解析
#[wasm_bindgen]
pub fn load(yaml: &str) -> Result<JsValue, JsValue> {
    let data: Value = serde_yaml::from_str(yaml).map_err(|e| JsValue::from_str(&e.to_string()))?;
    to_value(&data).map_err(|e| JsValue::from_str(&e.to_string()))
}

// 将数据结构序列化为 YAML
#[wasm_bindgen]
pub fn dump(data: &JsValue) -> Result<String, JsValue> {
    let obj: Value = from_value(data.clone()).map_err(|e| JsValue::from_str(&e.to_string()))?;
    serde_yaml::to_string(&obj).map_err(|e| JsValue::from_str(&e.to_string()))
}

// 合并数据
#[wasm_bindgen]
pub fn merge(data: &JsValue, patch: &JsValue) -> Result<JsValue, JsValue> {
    let mut data: Value =
        from_value(data.clone()).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let patch: Value = from_value(patch.clone()).map_err(|e| JsValue::from_str(&e.to_string()))?;
    merge_value(&mut data, &patch);
    to_value(&data).map_err(|e| JsValue::from_str(&e.to_string()))
}

fn merge_value(data: &mut Value, patch: &Value) {
    match (data, patch) {
        (Value::Object(data), Value::Object(patch)) => {
            for (key, value) in patch {
                if let Some(data_value) = data.get_mut(key) {
                    merge_value(data_value, value);
                } else {
                    data.insert(key.clone(), value.clone());
                }
            }
        }
        (data, patch) => {
            *data = patch.clone();
        }
    }
}
