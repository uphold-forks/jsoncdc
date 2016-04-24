use pg95 as pg;

use std::mem::size_of;

pub fn set_output_to_textual(ctx: *mut pg::Struct_LogicalDecodingContext,
                             options: *mut pg::OutputPluginOptions,
                             _is_init: pg::_bool) {
    unsafe {
        use pg95::Enum_OutputPluginOutputType::*;
        let last_relid = pg::palloc0(size_of::<pg::Oid>() as u64);
        (*ctx).output_plugin_private = last_relid;
        (*options).output_type = OUTPUT_PLUGIN_TEXTUAL_OUTPUT;
    }
}
