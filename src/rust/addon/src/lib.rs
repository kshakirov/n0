use n0_core::parse_http_header;
use n0_core::parse_http_req;
use n0_core::{BodyParserState, BodyParserStatus, DelimiterContext, NetworkInput};

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
    mut body_state: Int32Array,
) -> Result<()> {
    if header_state.len() < 3 || parser_state.is_empty() || body_state.len() < 5 {
        return Err(Error::from_reason("Недостаточная длина массива состояния"));
    }
    // body_state: [статус тела, сетевой режим, current_value, arena_ptr, delimiter_context].
    let state = match body_state[0] {
        0 => BodyParserStatus::ExpectChunkSize,
        1 => BodyParserStatus::ReadChunkData,
        2 => BodyParserStatus::ReadFixedData,
        3 => BodyParserStatus::ReadChunkSize,
        4 => BodyParserStatus::Success,
        5 => BodyParserStatus::Error,
        6 => BodyParserStatus::ExpectChunkCr,
        7 => BodyParserStatus::ExpectChunkLf,
        8 => BodyParserStatus::ExpectFinalCr,
        9 => BodyParserStatus::ExpectFinalLf,
        _ => return Err(Error::from_reason("Неизвестный статус парсера тела")),
    };
    let networking_input = match body_state[1] {
        0 => NetworkInput::ReadingFixedData,
        1 => NetworkInput::Timeout,
        2 => NetworkInput::NeedMoreData,
        3 => NetworkInput::ChunkSizeDigitAccepted,
        4 => NetworkInput::ZeroChunkSizeTerminated,
        5 => NetworkInput::ChunkSizeTerminated,
        6 => NetworkInput::CrAccepted,
        7 => NetworkInput::LfAccepted,
        8 => NetworkInput::FixedBodyByteAccepted,
        9 => NetworkInput::ChunkDataByteAccepted,
        10 => NetworkInput::InvalidInput,
        _ => return Err(Error::from_reason("Неизвестный сетевой режим")),
    };
    let current_value = usize::try_from(body_state[2])
        .map_err(|_| Error::from_reason("Отрицательный current_value"))?;
    let arena_ptr = usize::try_from(body_state[3])
        .map_err(|_| Error::from_reason("Отрицательный arena_ptr"))?;
    let bytes: &[u8] = buffer_1.as_ref();
    let arena: &mut [u8] = buffer_2.as_mut();
    let delimiter_context = match body_state[4] {
        0 => DelimiterContext::NoDelimiter,
        1 => DelimiterContext::AfterSize,
        2 => DelimiterContext::AfterData,
        3 => DelimiterContext::AfterZero,
        _ => return Err(Error::from_reason("Неизвестный контекст разделителя")),
    };
    let mut body_parser_state = BodyParserState {
        state,
        networking_input,
        current_value,
        arena,
        arena_ptr,
        delimiter_context,
    };
    parse_http_req(
        bytes,
        &mut header_state,
        &mut parser_state,
        &mut body_parser_state,
    );
    body_state[0] = body_parser_state.state as i32;
    body_state[1] = body_parser_state.networking_input as i32;
    body_state[2] = i32::try_from(body_parser_state.current_value)
        .map_err(|_| Error::from_reason("current_value не помещается в Int32Array"))?;
    body_state[3] = i32::try_from(body_parser_state.arena_ptr)
        .map_err(|_| Error::from_reason("arena_ptr не помещается в Int32Array"))?;
    body_state[4] = body_parser_state.delimiter_context as i32;
    Ok(())
}
