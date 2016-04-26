#[cfg(feature = "rpg94-sys")]
extern crate rpg94_sys as pg;

#[cfg(feature = "rpg95-sys")]
extern crate rpg95_sys as pg;

use std::ffi::CString;
use std::mem::size_of;

pub fn write_text1(ctx: *mut pg::Struct_LogicalDecodingContext,
                   text: &str,
                   t: u32) {
    let s = CString::new(text).unwrap();
    unsafe {
        pg::OutputPluginPrepareWrite(ctx, CTRUE);
        pg::appendStringInfo((*ctx).out, s.as_ptr(), t);
        pg::OutputPluginWrite(ctx, CTRUE);
    }
}

pub fn write_text2(ctx: *mut pg::Struct_LogicalDecodingContext,
                   text: &str,
                   t1: u32,
                   t2: *const i8) {
    let s = CString::new(text).unwrap();
    unsafe {
        pg::OutputPluginPrepareWrite(ctx, CTRUE);
        pg::appendStringInfo((*ctx).out, s.as_ptr(), t1, t2);
        pg::OutputPluginWrite(ctx, CTRUE);
    }
}

pub fn write_text2str(ctx: *mut pg::Struct_LogicalDecodingContext,
                      text: &str,
                      t1: u32,
                      t2: *const i8) {
    let s = CString::new(text).unwrap();
    unsafe {
        pg::OutputPluginPrepareWrite(ctx, CTRUE);
        pg::appendStringInfo((*ctx).out, s.as_ptr(), t1, t2);
        pg::OutputPluginWrite(ctx, CTRUE);
    }
}

pub fn set_output_to_textual(ctx: *mut pg::Struct_LogicalDecodingContext,
                             options: *mut pg::OutputPluginOptions,
                             _is_init: pg::_bool) {
    unsafe {
        (*options).output_type = pg::Enum_OutputPluginOutputType::OUTPUT_PLUGIN_TEXTUAL_OUTPUT;
    }
}


const CTRUE: pg::_bool = 1;
const CFALSE: pg::_bool = 0;
