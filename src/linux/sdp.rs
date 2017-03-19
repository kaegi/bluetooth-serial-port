extern crate enum_primitive;

use super::ffi::*;
use super::socket::create_error_from_errno;
use super::socket::create_error_from_last;

use bluetooth::{BtAddr, BtError};

use std::mem;
use std::ptr;
use std::slice;
use std::os::raw::*;
use std::os::unix;
use enum_primitive::FromPrimitive;

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
struct sdp_session_t {
    sock: unix::io::RawFd,
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

    fn sdp_service_search_attr_async(session: *mut sdp_session_t,
        search: *const sdp_list_t,
        reqtype: SdpAttrReqType,
        attrid_list: *const sdp_list_t) -> c_int;
    fn sdp_process(session: *mut sdp_session_t) -> c_int;
    fn sdp_get_error(session: *mut sdp_session_t) -> c_int;
    fn sdp_set_notify(session: *mut sdp_session_t, func: Option<unsafe extern "C" fn(u8, u16, *const u8, usize, *mut c_void)>, udata: *mut c_void) -> c_int;

    fn sdp_extract_seqtype(buf: *const u8, bufsize: c_int, dtdp: *mut u8, size: *mut c_int) -> c_int;
    fn sdp_extract_pdu(pdata: *const u8, bufsize: c_int, scanned: *mut c_int) -> *mut sdp_record_t;

    fn sdp_get_access_protos(rec: *const sdp_record_t, protos: *mut *mut sdp_list_t) -> c_int;
    fn sdp_uuid_to_proto(uuid: *mut uuid_t) -> c_int;

    fn sdp_list_free(list: *mut sdp_list_t, free_func: *const c_void);
    fn sdp_close(session: *mut sdp_session_t) -> c_int;
    fn sdp_record_free(rec: *mut sdp_record_t);
}


#[derive(Debug)]
enum QueryRFCOMMChannelState {
    New,
    Connecting,
    WaitForData,
    Done
}

#[derive(Debug)]
pub enum QueryRFCOMMChannelStatus {
    WaitReadable(unix::io::RawFd),
    WaitWritable(unix::io::RawFd),
    Done(u8)
}

#[derive(Debug)]
pub struct QueryRFCOMMChannel {
    addr:    BtAddr,
    session: *mut sdp_session_t,
    state:   QueryRFCOMMChannelState,
    
    response: Option<Result<u8, BtError>>
}
impl QueryRFCOMMChannel {
    pub fn new(addr: BtAddr) -> Self {
        QueryRFCOMMChannel {
            addr:    addr,
            session: ptr::null_mut(),
            state:   QueryRFCOMMChannelState::New,
            
            response: None
        }
    }
    
    unsafe extern "C" fn notify_cb(_: u8, status: u16, rsp: *const u8, size: usize, this_ptr: *mut c_void) {
        fn make_status_error(message: &str) -> BtError {
            BtError::Desc(format!("sdp_service_search_attr_async(): Protocol error: {}", message))
        }
        
        let this = &mut *(this_ptr as *mut Self);
        
        this.response = Some(match status {
            0 => Self::parse_response(slice::from_raw_parts(rsp, size)),
            
            0x0001 => // SDP_INVALID_VERSION
                Err(make_status_error("Invalid version")),
            0x0002 => // SDP_INVALID_RECORD_HANDLE
                Err(make_status_error("Invalid record handle")),
            0x0003 => // SDP_INVALID_SYNTAX
                Err(make_status_error("Invalid syntax")),
            0x0004 => // SDP_INVALID_PDU_SIZE
                Err(make_status_error("Invalid PDU size")),
            0x0005 => // SDP_INVALID_CSTATE
                Err(make_status_error("Invalid CState")),
            _      =>
                Err(create_error_from_errno(
                        "sdp_service_search_attr_async(): Service record search failed",
                        sdp_get_error(this.session)
                ))
        });
    }
    
    fn parse_response(response: &[u8]) -> Result<u8, BtError> {
        let mut data_type: u8 = 0;
        let mut seqlen: c_int = 0;

        // Response is a sequence of sequence(s) for one or
		// more data element sequence(s) representing services
		// for which attributes are returned
        let mut scanned = unsafe {
            sdp_extract_seqtype(response.as_ptr(), response.len() as i32, &mut data_type, &mut seqlen)
        };

        let mut channel: Option<u8> = None;
        if scanned > 0 && seqlen > 0 {
            let mut pdata     = unsafe { response.as_ptr().offset(scanned as isize) };
            let mut pdata_len = (response.len() as i32) - scanned;
            
            while scanned < (response.len() as i32) && pdata_len > 0 {
                let mut record_size: c_int = 0;
                let record = unsafe { sdp_extract_pdu(pdata, pdata_len, &mut record_size) };
                if record.is_null() {
                    return Err(BtError::Desc("sdp_extract_pdu() returned NULL during parsing".to_string()));
                } else if record_size < 1 {
                    unsafe { sdp_record_free(record) };
                    break;
                }

                scanned += record_size;
                pdata     = unsafe { pdata.offset(record_size as isize) };
                pdata_len = pdata_len - record_size;

                // get a list of the protocol sequences
                let mut proto_list: *mut sdp_list_t = ptr::null_mut();
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

                        unsafe { sdp_list_free( mem::transmute((*p).data) , ptr::null()) };
                        p = unsafe{*p}.next;
                    }

                    unsafe { sdp_list_free(proto_list, ptr::null()) };
                }

                unsafe { sdp_record_free(record) };
            }
        }

        match channel {
            Some(idx) =>
                Ok(idx),
            None =>
                Err(BtError::Desc("No RFCOMM service on remote device".to_string()))
        }
    }

    pub fn advance(&mut self) -> Result<QueryRFCOMMChannelStatus, BtError> {
        macro_rules! get_fd {
            () => {
                {
                    assert!(!self.session.is_null());
                    unsafe { (*self.session).sock }
                }
            }
        };

        match &self.state {
            &QueryRFCOMMChannelState::New => {
                let flags = SdpConnectFlags::NonBlocking as u32;
                self.session = unsafe { sdp_connect(&BtAddr::any(), &self.addr, flags) };
                if self.session == ptr::null_mut() {
                    return Err(create_error_from_last("sdp_connect(): Bluetooth device not accessible"));
                }

                self.state = QueryRFCOMMChannelState::Connecting;
                Ok(QueryRFCOMMChannelStatus::WaitWritable(get_fd!()))
            },

            &QueryRFCOMMChannelState::Connecting => {
                // specify the UUID of the application we're searching for
                let mut service_uuid = uuid_t::default();
                unsafe { sdp_uuid16_create(&mut service_uuid, SdpProfile::SerialPort as u16) };
                let search_list = unsafe { sdp_list_append(ptr::null_mut(), mem::transmute(&mut service_uuid)) };

                // specify that we want a list of all the matching applications' attributes
                let mut range = 0x0000FFFFu32;
                let attrid_list = unsafe { sdp_list_append(ptr::null_mut(), mem::transmute(&mut range)) };

                // register for notification once all data has been parsed
                let this_ptr: *mut Self = self;
                unsafe { sdp_set_notify(self.session, Some(Self::notify_cb), this_ptr as *mut c_void) };

                // get a list of service records that have the serial port UUID
                let result = unsafe {
                    let status = sdp_service_search_attr_async(self.session, search_list, SdpAttrReqType::Range, attrid_list);
                    if status < 0 {
                        Err(create_error_from_last("sdp_service_search_attr_async(): Sending service record search request failed"))
                    } else {
                        Ok(())
                    }
                };

                // make sure data is freed before acting upon the result of the previous operation
                unsafe { sdp_list_free(search_list, ptr::null()) };
                unsafe { sdp_list_free(attrid_list, ptr::null()) };

                // quit if sending service request failed
                try!(result);

                self.state = QueryRFCOMMChannelState::WaitForData;
                Ok(QueryRFCOMMChannelStatus::WaitReadable(get_fd!()))
            },

            &QueryRFCOMMChannelState::WaitForData => {
                let status = unsafe { sdp_process(self.session) };
                if status < 0 {
                    // Transaction completed – parsing function should have already been called
                    assert!(self.response.is_some());
                    
                    // Unregister callback function
                    unsafe { sdp_set_notify(self.session, None, ptr::null_mut()) };

                    // Close session
                    if unsafe { sdp_close(self.session) } < 0 {
                        return Err(create_error_from_last("sdp_close()"));
                    }

                    self.state = QueryRFCOMMChannelState::Done;
                    match self.response.take().unwrap() {
                        Ok(channel) =>
                            Ok(QueryRFCOMMChannelStatus::Done(channel)),
                        Err(error)  =>
                            Err(error)
                    }
                } else {
                    // Transaction ongoing
                    Ok(QueryRFCOMMChannelStatus::WaitReadable(get_fd!()))
                }
            },

            &QueryRFCOMMChannelState::Done => {
                panic!("Trying advance `QueryRFCOMMChannel` from `Done` state");
            }
        }
    }
}
