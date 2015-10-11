extern crate libc;

use self::libc::{c_char, c_int};

use super::BtAddr;

#[link(name="bluetooth")]
extern {
    pub fn str2ba(addr : *const libc::c_char, parsed_address : &mut BtAddr) -> c_int;
    pub fn ba2str(addr : &BtAddr, formatted_address : *const libc::c_char) -> c_int;
}
