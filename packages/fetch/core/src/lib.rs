use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys::Promise, Request, Response};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = globalThis, js_name = fetch)]
    fn fetch_with_request(request: &Request) -> Promise;
}

#[wasm_bindgen]
pub async fn fetch_url(url: String) -> Result<JsValue, JsValue> {
    // 创建一个 Request 对象
    let request = Request::new_with_str(&url)
        .map_err(|e| JsValue::from(e.as_string().unwrap_or_default()))?;

    // 发起 fetch 请求，使用全局的 fetch 函数
    let response_promise: Promise = fetch_with_request(&request);
    let response = JsFuture::from(response_promise).await?;

    // 将 JsValue 转换为 Response
    let response: Response = response
        .dyn_into()
        .map_err(|e| JsValue::from(e.as_string().unwrap_or_default()))?;

    // 检查响应是否成功
    if !response.ok() {
        return Err(JsValue::from_str("Request failed"));
    }

    // 获取响应文本
    let text_promise: Promise = response
        .text()
        .map_err(|e| JsValue::from(e.as_string().unwrap_or_default()))?;
    let text = JsFuture::from(text_promise).await?;

    Ok(text)
}
