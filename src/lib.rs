extern crate libc;

use serde::Deserialize;
use widestring::U16CStr;
use widestring::U16CString;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GeoEngine {
    client: reqwest::blocking::Client,
    endpoint: String,
}

#[repr(C)]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoLocation {
    x: libc::c_int,
    y: libc::c_int,
    z: libc::c_int,
}

#[repr(C)]
pub struct SliceLoc {
    ptr: *mut GeoLocation,
    len: libc::size_t,
}

#[no_mangle]
pub extern "cdecl" fn geo_engine(endpoint: *const u16) -> *mut GeoEngine {
    let client = reqwest::blocking::Client::new();
    let c_str: &U16CStr = unsafe { U16CStr::from_ptr_str(endpoint) };
    let endp = c_str.to_string_lossy();

    let b = Box::new(GeoEngine {
        client,
        endpoint: endp,
    });

    Box::<GeoEngine>::into_raw(b)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CanSeeResponse {
    can_see: bool,
}

#[no_mangle]
pub extern "cdecl" fn geo_can_see_target(
    engine: *mut GeoEngine,
    x: libc::c_int,
    y: libc::c_int,
    z: libc::c_int,
    tx: libc::c_int,
    ty: libc::c_int,
    tz: libc::c_int,
) -> libc::c_char {
    let endpoint: &String = unsafe { &(*engine).endpoint };

    let url = reqwest::Url::parse_with_params(
        &("".to_string() + endpoint + &"/canSeeTarget".to_string()),
        &[
            ("x", x.to_string()),
            ("y", y.to_string()),
            ("z", z.to_string()),
            ("tx", tx.to_string()),
            ("ty", ty.to_string()),
            ("tz", tz.to_string()),
        ],
    );

    unsafe {
        match url {
            Ok(url) => match (*engine).client.get(url).send() {
                Ok(response) => match response.json::<CanSeeResponse>() {
                    Ok(response) => response.can_see as i8,
                    Err(..) => -2,
                },
                Err(..) => -1,
            },
            Err(..) => -3,
        }
    }
}

#[no_mangle]
pub extern "cdecl" fn geo_find_path(
    engine: *mut GeoEngine,
    path: *mut SliceLoc,
    x: libc::c_int,
    y: libc::c_int,
    z: libc::c_int,
    tx: libc::c_int,
    ty: libc::c_int,
    tz: libc::c_int,
) -> libc::c_char {
    let endpoint: &String = unsafe { &(*engine).endpoint };

    let url = reqwest::Url::parse_with_params(
        &("".to_string() + endpoint + &"/findPath".to_string()),
        &[
            ("x", x.to_string()),
            ("y", y.to_string()),
            ("z", z.to_string()),
            ("tx", tx.to_string()),
            ("ty", ty.to_string()),
            ("tz", tz.to_string()),
        ],
    );

    unsafe {
        match url {
            Ok(url) => match (*engine).client.get(url).send() {
                Ok(response) => match response.json::<Vec<GeoLocation>>() {
                    Ok(response) => {
                        let boxed_slice: Box<[GeoLocation]> = response.into_boxed_slice();
                        let len = boxed_slice.len();
                        let fat_ptr: *mut [GeoLocation] = Box::into_raw(boxed_slice);
                        let slim_ptr: *mut GeoLocation = fat_ptr as _;
                        *path = SliceLoc { ptr: slim_ptr, len };

                        match len {
                            0 => 0,
                            _ => 1,
                        }
                    }
                    Err(..) => -3,
                },
                Err(..) => -2,
            },
            Err(..) => -1,
        }
    }
}

#[no_mangle]
pub unsafe extern "cdecl" fn geo_path_destroy(arg: SliceLoc) {
    let SliceLoc { ptr: _, len: _ } = arg;
}

#[no_mangle]
pub unsafe extern "cdecl" fn geo_destroy(engine: *mut GeoEngine) {
    let _ = Box::<GeoEngine>::from_raw(engine);
}

#[no_mangle]
pub unsafe extern "cdecl" fn free_str(n: *mut u16) {
    let _ = U16CString::from_raw(n);
}
