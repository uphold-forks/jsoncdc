#[cfg(pg94)]
pub use pg94 as pg;
#[cfg(pg95)]
pub use pg95 as pg;
use std::mem::size_of;

pub fn startup_helper(ctx: *mut pg::Struct_LogicalDecodingContext,
                      options: *mut pg::OutputPluginOptions,
                      _is_init: pg::_bool) {
    set_output_to_textual(ctx, options, _is_init)
}



fn set_output_to_textual(ctx: *mut pg::Struct_LogicalDecodingContext,
                         options: *mut pg::OutputPluginOptions,
                         _is_init: pg::_bool) {
    use pg::Enum_OutputPluginOutputType::*;
    unsafe {
        let last_relid = pg::palloc0(size_of::<pg::Oid>() as u64);
        (*ctx).output_plugin_private = last_relid;
        (*options).output_type = OUTPUT_PLUGIN_TEXTUAL_OUTPUT;
    }
}
