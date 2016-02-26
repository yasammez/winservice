use std::os::raw::{c_int, c_char, c_void};
use std::ffi::CString;
use std::sync::mpsc::Sender;
use service_status::SERVICE_STATUS;

#[link(name = "Advapi32")]
#[allow(improper_ctypes)]
extern "stdcall" {
    pub fn StartServiceCtrlDispatcherA(service_table : *const SERVICE_TABLE_ENTRY) -> c_int;
    pub fn RegisterServiceCtrlHandlerExA(lpServiceName : *const c_char,
        lpHandlerProc : extern "system" fn(CONTROL_CODE, u32, c_void, &Sender<()>) -> u32,
        lpContext : Box<Sender<()>>) -> *const SERVICE_STATUS_HANDLE;
    pub fn SetServiceStatus(hServiceStatus : *const SERVICE_STATUS_HANDLE,
        lpServiceStatus : *const SERVICE_STATUS) -> c_int;
}

#[allow(non_camel_case_types)]
pub enum SERVICE_STATUS_HANDLE {}

#[allow(non_camel_case_types, unused, non_snake_case)]
#[repr(u32)]
pub enum SERVICE_TYPE  {
    SERVICE_KERNEL_DRIVER = 0x00000001,
    SERVICE_FILE_SYSTEM_DRIVER = 0x00000002,
    SERVICE_WIN32_OWN_PROCESS = 0x00000010,
    SERVICE_WIN32_SHARE_PROCESS = 0x00000020,
    SERVICE_WIN32 = 0x00000030,
    SERVICE_INTERACTIVE_PROCESS = 0x00000100,
}

#[allow(non_camel_case_types, unused, non_snake_case)]
#[repr(u32)]
pub enum ACCEPTED_CONTROLS {
    SERVICE_ACCEPT_STOP = 0x00000001,
    SERVICE_ACCEPT_PAUSE_CONTINUE = 0x00000002,
    SERVICE_ACCEPT_SHUTDOWN = 0x00000004,
    SERVICE_ACCEPT_STOP_SHUTDOWN = 0x00000005,
    SERVICE_ACCEPT_PARAMCHANGE = 0x00000008,
    SERVICE_ACCEPT_NETBINDCHANGE = 0x00000010,
    SERVICE_ACCEPT_PRESHUTDOWN = 0x00000100,
}

#[allow(non_camel_case_types, unused, non_snake_case)]
#[derive(PartialEq)]
#[repr(u32)]
pub enum CURRENT_SERVICE_STATUS {
    SERVICE_STOPPED = 0x00000001,
    SERVICE_START_PENDING = 0x00000002,
    SERVICE_STOP_PENDING = 0x00000003,
    SERVICE_RUNNING = 0x00000004,
    SERVICE_CONTINUE_PENDING = 0x00000005,
    SERVICE_PAUSE_PENDING = 0x00000006,
    SERVICE_PAUSED = 0x00000007,
}

#[allow(non_camel_case_types, unused, non_snake_case)]
#[repr(u32)]
pub enum CONTROL_CODE {
    SERVICE_CONTROL_STOP = 0x00000001,
    SERVICE_CONTROL_PAUSE = 0x00000002,
    SERVICE_CONTROL_CONTINUE = 0x00000003,
    SERVICE_CONTROL_INTERROGATE = 0x00000004,
    SERVICE_CONTROL_SHUTDOWN = 0x00000005,
    SERVICE_CONTROL_PARAMCHANGE = 0x00000006,
    SERVICE_CONTROL_NETBINDADD = 0x00000007,
    SERVICE_CONTROL_NETBINDREMOVE = 0x00000008,
    SERVICE_CONTROL_NETBINDENABLE = 0x00000009,
    SERVICE_CONTROL_NETBINDDISABLE = 0x0000000A,
}

#[allow(non_camel_case_types, unused, non_snake_case)]
#[repr(C)]
pub struct SERVICE_TABLE_ENTRY {
    lpServiceName : *const c_char,
    lpServiceProc : Option<extern fn(c_int, *const *const c_char) -> ()>
}

impl SERVICE_TABLE_ENTRY {
    pub fn with_wrapper(wrapper: extern "C" fn(c_int, *const *const c_char))
        -> [SERVICE_TABLE_ENTRY; 2]
    {
        [
            SERVICE_TABLE_ENTRY {
                lpServiceName : CString::new("").unwrap().as_ptr(),
                lpServiceProc : Some(wrapper) },
            SERVICE_TABLE_ENTRY {
                lpServiceName : 0 as *const c_char,
                lpServiceProc : None }
        ]
    }
}

