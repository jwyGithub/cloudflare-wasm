use serde_json::Value;

#[no_mangle]
pub extern "C" fn alloc(len: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut u8, len: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, 0, len);
    }
}

#[no_mangle]
pub extern "C" fn load(ptr: *const u8, len: usize) -> *mut u8 {
    let yaml = unsafe { std::str::from_utf8(std::slice::from_raw_parts(ptr, len)).unwrap() };
    let result = match serde_yaml::from_str::<Value>(yaml) {
        Ok(value) => serde_json::to_string(&value).unwrap(),
        Err(e) => format!("Error: {}", e),
    };
    let bytes = result.into_bytes();
    let output_ptr = bytes.as_ptr() as *mut u8;
    std::mem::forget(bytes);
    output_ptr
}

#[no_mangle]
pub extern "C" fn dump(ptr: *const u8, len: usize) -> *mut u8 {
    let json = unsafe { std::str::from_utf8(std::slice::from_raw_parts(ptr, len)).unwrap() };
    let result = match serde_json::from_str::<Value>(json) {
        Ok(value) => serde_yaml::to_string(&value).unwrap(),
        Err(e) => format!("Error: {}", e),
    };
    let bytes = result.into_bytes();
    let output_ptr = bytes.as_ptr() as *mut u8;
    std::mem::forget(bytes);
    output_ptr
}

#[no_mangle]
pub extern "C" fn merge(
    data_ptr: *const u8,
    data_len: usize,
    patch_ptr: *const u8,
    patch_len: usize,
) -> *mut u8 {
    let data =
        unsafe { std::str::from_utf8(std::slice::from_raw_parts(data_ptr, data_len)).unwrap() };
    let patch =
        unsafe { std::str::from_utf8(std::slice::from_raw_parts(patch_ptr, patch_len)).unwrap() };

    let result = match (
        serde_json::from_str::<Value>(data),
        serde_json::from_str::<Value>(patch),
    ) {
        (Ok(mut data_value), Ok(patch_value)) => {
            merge_value(&mut data_value, &patch_value);
            serde_json::to_string(&data_value).unwrap()
        }
        (Err(e), _) => format!("Error in data: {}", e),
        (_, Err(e)) => format!("Error in patch: {}", e),
    };

    let bytes = result.into_bytes();
    let output_ptr = bytes.as_ptr() as *mut u8;
    std::mem::forget(bytes);
    output_ptr
}

fn merge_value(data: &mut Value, patch: &Value) {
    match (data, patch) {
        (Value::Object(data_map), Value::Object(patch_map)) => {
            for (key, value) in patch_map {
                if let Some(data_value) = data_map.get_mut(key) {
                    merge_value(data_value, value);
                } else {
                    data_map.insert(key.clone(), value.clone());
                }
            }
        }
        (data, patch) => {
            *data = patch.clone();
        }
    }
}

// 必须的 WASI 入口点
#[no_mangle]
pub extern "C" fn _start() {}
