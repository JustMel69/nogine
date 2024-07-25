#![allow(non_camel_case_types)]

#[cfg(feature = "al_debug")]
macro_rules! al_call {
    ($api:expr, $x:expr) => {
        unsafe {
            $crate::audio::al::al_clear_error(&$api);
            let res = $x;
            $crate::audio::al::al_print_err(&$api, &format!("File {}, Ln {}, Col {}", file!(), line!(), column!()));
            res
        } 
    };
}

#[cfg(not(feature = "al_debug"))]
macro_rules! al_call {
    ($api:expr, $x:expr) => {
        unsafe { $x } 
    };
}

pub(crate) use al_call;

#[cfg(feature = "al_debug")]
pub unsafe fn al_clear_error(api: &al_sys::AlApi) {
    use al_sys::AlApi;

    while api.alGetError() != al_sys::AL_NO_ERROR {}
}

#[cfg(feature = "al_debug")]
pub unsafe fn al_print_err(api: &al_sys::AlApi, print_metadata: &str) {
    loop {
        let err = api.alGetError();
        if err == al_sys::AL_NO_ERROR {
            return;
        }

        let err = match err {
            al_sys::AL_INVALID_NAME => "AL_INVALID_NAME",
            al_sys::AL_INVALID_ENUM => "AL_INVALID_ENUM",
            al_sys::AL_INVALID_VALUE => "AL_INVALID_VALUE",
            al_sys::AL_INVALID_OPERATION => "AL_INVALID_OPERATION",
            al_sys::AL_OUT_OF_MEMORY => "AL_OUT_OF_MEMORY",
            _ => unreachable!()
        };

        eprintln!("{print_metadata}\t\t AL_ERROR: {err}");
    }
}