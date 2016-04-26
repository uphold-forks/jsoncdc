#[cfg(feature = "rpg94-sys")]
extern crate rpg94_sys as pg;

#[cfg(feature = "rpg95-sys")]
extern crate rpg95_sys as pg;

pub fn get_rel_name(nspid: pg::Oid) -> *mut ::std::os::raw::c_char {
    unsafe {
        pg::get_rel_name(nspid)
    }
}
