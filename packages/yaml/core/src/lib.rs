use js_sys::Map;
use serde_json::Value;
use serde_wasm_bindgen::{from_value, to_value};
use serde_yaml;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

// 从 YAML 字符串解析
#[wasm_bindgen]
pub fn load(yaml: &str) -> Result<Map, JsValue> {
    let data: Value = serde_yaml::from_str(yaml).map_err(|e| JsValue::from_str(&e.to_string()))?;
    value_to_js_map(&data)
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

// 将 serde_json::Value 转换为 JavaScript Map
fn value_to_js_map(value: &Value) -> Result<Map, JsValue> {
    let map = Map::new();

    if let Value::Object(obj) = value {
        for (key, val) in obj {
            // 假设嵌套的也是对象，因此递归转换为 Map
            if let Value::Object(_) = val {
                let nested_map = value_to_js_map(val)?;
                map.set(&JsValue::from_str(key), &JsValue::from(nested_map));
            } else {
                let js_val = value_to_js_value(val)?;
                map.set(&JsValue::from_str(key), &js_val);
            }
        }
        Ok(map)
    } else {
        Err(JsValue::from_str("Expected a JSON object"))
    }
}

// 将 serde_json::Value 转换为 JsValue
fn value_to_js_value(value: &Value) -> Result<JsValue, JsValue> {
    match value {
        Value::Null => Ok(JsValue::NULL),
        Value::Bool(b) => Ok(JsValue::from_bool(*b)),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(JsValue::from_f64(i as f64))
            } else if let Some(f) = n.as_f64() {
                Ok(JsValue::from_f64(f))
            } else {
                Err(JsValue::from_str("Invalid number"))
            }
        }
        Value::String(s) => Ok(JsValue::from_str(s)),
        Value::Array(arr) => {
            let js_array = js_sys::Array::new();
            for v in arr {
                js_array.push(&value_to_js_value(v)?);
            }
            Ok(JsValue::from(js_array))
        }
        Value::Object(_) => {
            let map = value_to_js_map(value)?;
            Ok(JsValue::from(map))
        }
    }
}
