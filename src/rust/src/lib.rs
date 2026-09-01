use std::ffi::c_void;
use std::ptr;

type NapiEnv = *mut c_void;
type NapiValue = *mut c_void;
type NapiCallbackInfo = *mut c_void;
type NapiStatus = i32;

const NAPI_OK: NapiStatus = 0;
const ISBUFFER: &[u8] = b"isBuffer\0";

unsafe extern "C" {
    fn napi_create_function(
        env: NapiEnv,
        name: *const u8,
        length: usize,
        callback: extern "C" fn(NapiEnv, NapiCallbackInfo) -> NapiValue,
        data: *mut c_void,
        result: *mut NapiValue,
    ) -> NapiStatus;

    fn napi_set_named_property(
        env: NapiEnv,
        object: NapiValue,
        name: *const u8,
        value: NapiValue,
    ) -> NapiStatus;

    fn napi_get_cb_info(
        env: NapiEnv,
        info: NapiCallbackInfo,
        argc: *mut usize,
        argv: *mut NapiValue,
        this_arg: *mut NapiValue,
        data: *mut *mut c_void,
    ) -> NapiStatus;
    fn napi_is_buffer(env: NapiEnv, value: NapiValue, result: *mut bool) -> NapiStatus;

    fn napi_get_boolean(env: NapiEnv, value: bool, result: *mut NapiValue) -> NapiStatus;

}

extern "C" fn is_buffer(env: NapiEnv, info: NapiCallbackInfo) -> NapiValue {
    let mut arguments = [ptr::null_mut()];
    let mut argument_count = arguments.len();
    let mut result = false;
    let mut n_result: NapiValue = ptr::null_mut();

    if unsafe {
        napi_get_cb_info(
            env,
            info,
            &mut argument_count,
            arguments.as_mut_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
        )
    } != NAPI_OK
        || argument_count != 1
    {
        return ptr::null_mut();
    }
    if unsafe { napi_is_buffer(env, arguments[0], &mut result) } != NAPI_OK {
        return ptr::null_mut();
    }
    if unsafe { napi_get_boolean(env, result, &mut n_result) } != NAPI_OK {
        return ptr::null_mut();
    }

    n_result
}

#[unsafe(no_mangle)]
pub extern "C" fn napi_register_module_v1(env: NapiEnv, exports: NapiValue) -> NapiValue {
    let mut function = ptr::null_mut();

    if unsafe {
        napi_create_function(
            env,
            ISBUFFER.as_ptr(),
            ISBUFFER.len() - 1,
            is_buffer,
            ptr::null_mut(),
            &mut function,
        )
    } != NAPI_OK
    {
        return ptr::null_mut();
    }

    if unsafe { napi_set_named_property(env, exports, ISBUFFER.as_ptr(), function) } != NAPI_OK {
        return ptr::null_mut();
    }

    exports
}
