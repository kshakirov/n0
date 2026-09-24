use n0_core::parse_http_header;
use napi::bindgen_prelude::*;
use napi_derive::napi;
#[napi(js_name = "isBuffer")]
pub fn process_buffer_macro(
    mut buffer: napi::bindgen_prelude::Buffer,
) -> napi::bindgen_prelude::Buffer {
    // Работаем напрямую с безопасным Rust-срезом:
    let bytes: &mut [u8] = buffer.as_mut();

    for b in bytes {
        *b = 42; // Для примера меняем байты на 42
    }

    // Возвращаем тот же буфер в JS без копирования
    buffer
}

#[napi(js_name = "n_parse_http_header")]
pub fn n_parse_http_header(
    buffer: napi::bindgen_prelude::Buffer,
    mut numbers: Int32Array,
) -> napi::bindgen_prelude::Buffer {
    let bytes: &[u8] = buffer.as_ref();
    parse_http_header(bytes, &mut numbers);
    buffer
}
