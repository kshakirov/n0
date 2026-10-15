use n0_core::parse_http_header;
use n0_core::parse_http_req;

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
pub fn n_parse_http_header(buffer: napi::bindgen_prelude::Buffer, mut numbers: Int32Array) {
    let bytes: &[u8] = buffer.as_ref();
    parse_http_header(bytes, &mut numbers);
}

#[napi(js_name = "n_parse_http_req")]
pub fn n_parse_http_req(
    buffer_1: napi::bindgen_prelude::Buffer,
    mut buffer_2: napi::bindgen_prelude::Buffer,
    mut header_state: Int32Array,
    mut parser_state: Int32Array,
) {
    let bytes: &[u8] = buffer_1.as_ref();
    let arena: &mut [u8] = buffer_2.as_mut();
    parse_http_req(bytes, arena, &mut header_state, &mut parser_state);
}
