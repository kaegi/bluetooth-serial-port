extern crate enum_primitive;

use super::ffi::*;

use bluetooth::{BtAddr, BtError};

use std::ptr;
use std::os::raw::*;
use std::mem;
use enum_primitive::FromPrimitive;

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
struct sdp_session_t {
    sock: c_int,
    state: c_int,
    local: c_int,
    flags: c_int,
    tid: uint16_t,
    priv_: *mut c_void,
}
impl ::std::default::Default for sdp_session_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
struct uuid_union_t {
    _bindgen_data_: [u32; 4usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
struct uuid_t {
    pub type_: uint8_t,
    pub value: uuid_union_t,
}
impl ::std::default::Default for uuid_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
struct sdp_list_t {
    next: *mut sdp_list_t,
    data: *mut c_void,
}
impl ::std::default::Default for sdp_list_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[allow(dead_code)]
pub enum SdpAttrReqType {
    Individual = 1,
    Range = 2,
}


#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
struct sdp_record_t {
    pub handle: uint32_t,
    pub pattern: *mut sdp_list_t,
    pub attrlist: *mut sdp_list_t,
    pub svclass: uuid_t,
    _bindgen_padding_0_: [u8; 4usize],
}
impl ::std::default::Default for sdp_record_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}


#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
struct sdp_val_union_t {
    _bindgen_data_: [u64; 3usize],
}
impl sdp_val_union_t {
    unsafe fn uint8(&mut self) -> *mut uint8_t {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    unsafe fn uuid(&mut self) -> *mut uuid_t {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
struct sdp_data_t {
    dtd: uint8_t,
    attr_id: uint16_t,
    val: sdp_val_union_t,
    next: *mut sdp_data_t,
    unit_size: c_int,
    _bindgen_padding_0_: [u8; 4usize],
}
impl ::std::default::Default for sdp_data_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

enum_from_primitive!{
    enum SdpPdu {
        DataNil            = 0x00,
        Uint8               = 0x08,
        Uint16              = 0x09,
        Uint32              = 0x0A,
        Uint64              = 0x0B,
        Uint128             = 0x0C,
        Int8                = 0x10,
        Int16               = 0x11,
        Int32               = 0x12,
        Int64               = 0x13,
        Int128              = 0x14,
        UuidUnspec         = 0x18,
        Uuid16              = 0x19,
        Uuid32              = 0x1A,
        Uuid128             = 0x1C,
        TextStrUnspec     = 0x20,
        TextStr8           = 0x25,
        TextStr16          = 0x26,
        TextStr32          = 0x27,
        Bool                = 0x28,
        SeqUnspec          = 0x30,
        Seq8                = 0x35,
        Seq16               = 0x36,
        Seq32               = 0x37,
        AltUnspec          = 0x38,
        Alt8                = 0x3D,
        Alt16               = 0x3E,
        Alt32               = 0x3F,
        UrlStrUnspec      = 0x40,
        UrlStr8            = 0x45,
        UrlStr16           = 0x46,
        UrlStr32           = 0x47,
    }
}

#[allow(dead_code)]
enum SdpConnectFlags {
    RetryIfBusy = 0x01,
    WaitOnClose = 0x02,
    NonBlocking = 0x04,
    LargeMtu    = 0x08,
}

enum SdpProfile {
    SerialPort = 0x1101,
}

enum SdpProtoUuid {
    Rfcomm = 0x0003,
}

#[cfg(target_os = "linux")]
#[link(name="bluetooth")]
extern "C" {
    fn sdp_connect(src: *const BtAddr, dst: *const BtAddr, flags: uint32_t) -> *mut sdp_session_t;
    fn sdp_uuid16_create(uuid: *mut uuid_t, data: uint16_t) -> *mut uuid_t;
    fn sdp_list_append(list: *mut sdp_list_t, d: *mut c_void) -> *mut sdp_list_t;
    fn sdp_service_search_attr_req(session: *mut sdp_session_t,
        search: *const sdp_list_t,
        reqtype: SdpAttrReqType,
        attrid_list: *const sdp_list_t,
        rsp_list: *mut *mut sdp_list_t) -> *mut sdp_record_t;

    fn sdp_get_access_protos(rec: *const sdp_record_t,
    protos: *mut *mut sdp_list_t) -> c_int;

    fn sdp_uuid_to_proto(uuid: *mut uuid_t) -> c_int;

    fn sdp_list_free(list: *mut sdp_list_t, free_func: *const c_void);
    fn sdp_close(session: *mut sdp_session_t) -> c_int;
    fn sdp_record_free(rec: *mut sdp_record_t);
}

pub fn get_rfcomm_channel(addr: BtAddr) -> Result<u8, BtError> {
    let mut channel: Option<u8> = None;

    let session = unsafe { sdp_connect(&BtAddr::any(), &addr, SdpConnectFlags::RetryIfBusy as u32) };
    if session == ptr::null_mut() { return Err(BtError::Desc("sdp_connect() failed".to_string())) }

    // specify the UUID of the application we're searching for
    let mut service_uuid = uuid_t::default();
    unsafe { sdp_uuid16_create(&mut service_uuid, SdpProfile::SerialPort as u16) };
    let search_list = unsafe { sdp_list_append(ptr::null_mut(), mem::transmute(&mut service_uuid)) };

    // specify that we want a list of all the matching applications' attributes
    let mut range = 0x0000FFFFu32;
    let attrid_list = unsafe { sdp_list_append(ptr::null_mut(), mem::transmute(&mut range)) };


    // get a list of service records that have the serial port UUID
    let mut response_list: *mut sdp_list_t = ptr::null_mut();
    unsafe { sdp_service_search_attr_req( session, search_list, SdpAttrReqType::Range, attrid_list, &mut response_list) };

    // go through each of the service records
    let mut r = response_list;
    while r != ptr::null_mut() {
        let record: *mut sdp_record_t = unsafe { mem::transmute((*r).data) };
        let mut proto_list: *mut sdp_list_t = ptr::null_mut();

        // get a list of the protocol sequences
        if unsafe { sdp_get_access_protos(record, &mut proto_list) } == 0 {
            let mut p = proto_list;

            while p != ptr::null_mut() {
                let mut pds: *mut sdp_list_t = unsafe { mem::transmute((*p).data) };

                // go through each protocol list of the protocol sequence
                while pds != ptr::null_mut() {

                    // check the protocol attributes
                    let mut d: *mut sdp_data_t = unsafe { mem::transmute((*pds).data) };
                    let mut proto: Option<c_int> = None;
                    while d != ptr::null_mut() {
                        match SdpPdu::from_u8(unsafe{*d}.dtd).unwrap_or_else(|| /* something that does not do anything = */ SdpPdu::DataNil) {
                            SdpPdu::Uuid16 |
                            SdpPdu::Uuid32 |
                            SdpPdu::Uuid128 => {
                                proto = Some(unsafe{sdp_uuid_to_proto((*d).val.uuid())});
                            }
                            SdpPdu::Uint8 => {
                                if proto == Some(SdpProtoUuid::Rfcomm as c_int) {
                                    channel = Some(unsafe{*(*d).val.uint8()});
                                }
                            }
                            _ => { }
                        }
                        d = unsafe{*d}.next;
                    }

                    pds = unsafe{*pds}.next;
                }

                unsafe{ sdp_list_free( mem::transmute((*p).data) , ptr::null()) };
                p = unsafe{*p}.next;
            }

            unsafe{ sdp_list_free(proto_list, ptr::null()) };
        }

        r = unsafe{*r}.next;
        unsafe{ sdp_record_free(record); }
    }

    unsafe{ sdp_close(session) };

    match channel {
        Some(x) => return Ok(x),
        None => Err(BtError::Desc("no RFCOMM service on remote device".to_string()))
    }
}
