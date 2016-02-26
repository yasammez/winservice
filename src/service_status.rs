use std::os::raw::c_void;
use std::ffi::CString;
use std::sync::mpsc::{Sender, Receiver, channel};
use windows::*;

#[allow(non_camel_case_types, unused, non_snake_case)]
#[repr(C)]
pub struct SERVICE_STATUS {
    dwServiceType : SERVICE_TYPE,
    dwCurrentState : CURRENT_SERVICE_STATUS,
    dwControlsAccepted : ACCEPTED_CONTROLS,
    dwWin32ExitCode : u32,
    dwServiceSpecificExitCode : u32,
    dwCheckPoint : u32,
    dwWaitHint : u32,
    /// This is not part of the official MSDN SERVICE_STATUS struct.
    handle : *const SERVICE_STATUS_HANDLE,
}

impl SERVICE_STATUS {

    pub fn initialize(name : &str) -> (Self, Receiver<()>) {
        let (tx, rx) = channel();
        let handle = unsafe { RegisterServiceCtrlHandlerExA(
            CString::new(name).unwrap().as_ptr(), Self::control_handler_ex, Box::new(tx)) };
        let mut context = SERVICE_STATUS::new(handle);
        context.set_status(CURRENT_SERVICE_STATUS::SERVICE_START_PENDING);
        context.set_status(CURRENT_SERVICE_STATUS::SERVICE_RUNNING);
        (context, rx)
    }

    pub fn set_status(&mut self, status : CURRENT_SERVICE_STATUS) {
        if status != self.dwCurrentState {
            self.dwCurrentState = status;
            unsafe { SetServiceStatus(self.handle, self as *const Self); }
        }
    }

    pub fn exit_with(&mut self, code : u32) {
        self.set_status(CURRENT_SERVICE_STATUS::SERVICE_STOP_PENDING);
        self.dwWin32ExitCode = code;
        self.set_status(CURRENT_SERVICE_STATUS::SERVICE_STOPPED);
    }

    fn new(handle : *const SERVICE_STATUS_HANDLE) -> SERVICE_STATUS {
        SERVICE_STATUS {
            dwServiceType : SERVICE_TYPE::SERVICE_WIN32,
            dwCurrentState : CURRENT_SERVICE_STATUS::SERVICE_STOPPED,
            dwControlsAccepted : ACCEPTED_CONTROLS::SERVICE_ACCEPT_STOP_SHUTDOWN,
            dwWin32ExitCode : 0,
            dwServiceSpecificExitCode : 0,
            dwCheckPoint : 0,
            dwWaitHint : 10000,
            handle : handle,
        }
    }

    #[allow(unused, non_snake_case)]
    extern "system" fn control_handler_ex(dwControl : CONTROL_CODE, dwEventType : u32,
        lpEventData : c_void, tx : &Sender<()>) -> u32
    {
        match dwControl {
            CONTROL_CODE::SERVICE_CONTROL_STOP | CONTROL_CODE::SERVICE_CONTROL_SHUTDOWN => unsafe {
                tx.send(()); },
            _ => {}
        }
        0
    }
}
