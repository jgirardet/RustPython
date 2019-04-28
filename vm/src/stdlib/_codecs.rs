use crate::function::OptionalArg;
use crate::obj::objbyteinner::ByteInnerDecodeOptions;
use crate::obj::objbytes::PyBytesRef;
use crate::obj::objstr::PyStringRef;
use crate::pyobject::PyObjectRef;
use crate::pyobject::PyResult;
use crate::vm::VirtualMachine;
use std::fmt::Write;
use std::str;
use std::string::FromUtf8Error;

// method to implement to replace _codecsmodule.c
// methods imported by codec.py

// _CODECS_REGISTER_METHODDEF
// _CODECS_LOOKUP_METHODDEF
// _CODECS_ENCODE_METHODDEF
// _CODECS_DECODE_METHODDEF
// _CODECS_ESCAPE_ENCODE_METHODDEF
// _CODECS_ESCAPE_DECODE_METHODDEF
// _CODECS_UTF_8_ENCODE_METHODDEF
// _CODECS_UTF_8_DECODE_METHODDEF : Done
// _CODECS_UTF_7_ENCODE_METHODDEF
// _CODECS_UTF_7_DECODE_METHODDEF
// _CODECS_UTF_16_ENCODE_METHODDEF
// _CODECS_UTF_16_LE_ENCODE_METHODDEF
// _CODECS_UTF_16_BE_ENCODE_METHODDEF
// _CODECS_UTF_16_DECODE_METHODDEF
// _CODECS_UTF_16_LE_DECODE_METHODDEF
// _CODECS_UTF_16_BE_DECODE_METHODDEF
// _CODECS_UTF_16_EX_DECODE_METHODDEF
// _CODECS_UTF_32_ENCODE_METHODDEF
// _CODECS_UTF_32_LE_ENCODE_METHODDEF
// _CODECS_UTF_32_BE_ENCODE_METHODDEF
// _CODECS_UTF_32_DECODE_METHODDEF
// _CODECS_UTF_32_LE_DECODE_METHODDEF
// _CODECS_UTF_32_BE_DECODE_METHODDEF
// _CODECS_UTF_32_EX_DECODE_METHODDEF
// _CODECS_UNICODE_ESCAPE_ENCODE_METHODDEF
// _CODECS_UNICODE_ESCAPE_DECODE_METHODDEF
// _CODECS_UNICODE_INTERNAL_ENCODE_METHODDEF
// _CODECS_UNICODE_INTERNAL_DECODE_METHODDEF
// _CODECS_RAW_UNICODE_ESCAPE_ENCODE_METHODDEF
// _CODECS_RAW_UNICODE_ESCAPE_DECODE_METHODDEF
// _CODECS_LATIN_1_ENCODE_METHODDEF
// _CODECS_LATIN_1_DECODE_METHODDEF
// _CODECS_ASCII_ENCODE_METHODDEF
// _CODECS_ASCII_DECODE_METHODDEF
// _CODECS_CHARMAP_ENCODE_METHODDEF
// _CODECS_CHARMAP_DECODE_METHODDEF
// _CODECS_CHARMAP_BUILD_METHODDEF
// _CODECS_READBUFFER_ENCODE_METHODDEF
// _CODECS_MBCS_ENCODE_METHODDEF
// _CODECS_MBCS_DECODE_METHODDEF
// _CODECS_OEM_ENCODE_METHODDEF
// _CODECS_OEM_DECODE_METHODDEF
// _CODECS_CODE_PAGE_ENCODE_METHODDEF
// _CODECS_CODE_PAGE_DECODE_METHODDEF
// _CODECS_REGISTER_ERROR_METHODDEF
// _CODECS_LOOKUP_ERROR_METHODDEF
// _CODECS__FORGET_CODEC_METHODDEF
// {NULL, NULL}                /* sentinel */
// cfg!(target_endian = "little")

//same algorithm as cpython
pub fn normalize_encoding(encoding: &str) -> String {
    let mut res = String::new();
    let mut punct = false;

    for c in encoding.chars() {
        if c.is_alphanumeric() || c == '.' {
            if punct && !res.is_empty() {
                res.push('_')
            }
            res.push(c.to_ascii_lowercase());
            punct = false;
        } else {
            punct = true;
        }
    }
    res
}

fn utf8_error_strict(err: FromUtf8Error, codec: &str, vm: &VirtualMachine) -> PyResult<String> {
    let pos = err.utf8_error().valid_up_to();
    Err(vm.new_unicode_error(format!(
        "{} codec can't decode byte \\x{:x} in position {}: invalid start byte",
        codec,
        err.as_bytes()[pos],
        pos
    )))
}

fn utf8_errors(tab: &[u8], errors: &str, vm: &VirtualMachine) -> PyResult<String> {
    let mut start = 0;
    let mut new_string = String::new();

    loop {
        let new_slice = &tab[start..];
        match std::str::from_utf8(new_slice) {
            Ok(value) => {
                new_string.push_str(&value);
                break;
            }
            Err(err) => {
                // safe since the range the previous range is already checked
                new_string
                    .push_str(unsafe { str::from_utf8_unchecked(&new_slice[..err.valid_up_to()]) });
                match errors {
                    "backslashreplace" => {
                        for &byte in &new_slice
                            [err.valid_up_to()..err.valid_up_to() + err.error_len().unwrap()]
                        {
                            write!(&mut new_string, "\\x{:x}", byte).expect("Unable to write");
                        }
                    }
                    "surrogateescape" => {
                        return Err(vm.new_not_implemented_error(
                            "surrogetescape isn\"t yet implemented in rustpython".to_string(),
                        ));
                    }
                    _ => {} // ignore remaining
                }
                start += err.valid_up_to() + err.error_len().unwrap();
            }
        }
    }
    Ok(new_string)
}

pub fn _utf_8_decode(bytes: &[u8], errors: &str, vm: &VirtualMachine) -> PyResult<String> {
    match errors {
        "strict" => match String::from_utf8(bytes.to_vec()) {
            Ok(value) => Ok(value),
            Err(err) => utf8_error_strict(err, "'utf-8'", vm),
        },
        "replace" => Ok(String::from_utf8_lossy(bytes).to_string()),
        "ignore" | "backslashreplace" | "surrogateescape" => utf8_errors(bytes, errors, vm),
        unknown => Err(vm.new_lookup_error(format!("unknown error handler name {}", unknown))),
    }
}

pub fn utf_8_decode2(
    obj: PyBytesRef,
    errorss: OptionalArg<PyStringRef>,
    vm: &VirtualMachine,
) -> PyResult<String> {
    let bytes = obj.get_value();
    let errors = if let OptionalArg::Present(value) = &errorss {
        value.as_str()
    } else {
        "strict"
    };
    match errors {
        "strict" => match String::from_utf8(bytes.to_vec()) {
            Ok(value) => Ok(value),
            Err(err) => utf8_error_strict(err, "'utf-8'", vm),
        },
        "replace" => Ok(String::from_utf8_lossy(bytes).to_string()),
        "ignore" | "backslashreplace" | "surrogateescape" => utf8_errors(bytes, errors, vm),
        unknown => Err(vm.new_lookup_error(format!("unknown error handler name {}", unknown))),
    }
}

pub fn make_module(vm: &VirtualMachine) -> PyObjectRef {
    let ctx = &vm.ctx;

    py_module!(vm, "_codecs", {
        "utf_8_decode" => ctx.new_rustfunc(utf_8_decode2),
    })
}
