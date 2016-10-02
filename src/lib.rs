//! Run a Windows system service without hassle.
//!
//! This crate exports two methods, but they should not be called directly from user code!
//! Instead use the provided `Service!` macro. The exports are necessary because there is no
//! way to provide the ServiceMain callback with a custom user pointer which could be used to
//! extract its context. Funnily, this **is** possible for the ControlHandlerEx callback, so at
//! least we only have to use this dirty trick once.
#![cfg(target_os = "windows")]

mod windows;
mod service_status;

use std::ffi::CStr;
use std::os::raw::{c_int, c_char};
use std::sync::mpsc::Receiver;
use windows::{SERVICE_TABLE_ENTRY, StartServiceCtrlDispatcherA};
use service_status::SERVICE_STATUS;

/// Create and run the provided function as Windows system service.
///
/// This takes the service name as a `str` expression and the function which contains the service's
/// main loop as arguments and immediately starts the service. Once this macro returns, the Service
/// Control Manager (SCM) has stopped the service and your program may terminate. The service
/// function gets a `Vec<String>` containing the arguments provided by the SCM (these are **not**
/// the command line arguments of your EXE!) as well as a `Receiver<()>` which, when signalled,
/// indicates that the SCM wants the service to stop. When that happens, the service function
/// should return a `u32` exit code, which will prompt this crate to perform some cleanup and
/// return from `Service!`.
///
/// To actually run the service, you have to install your binary from an Administrator console
/// window with
///
/// ```text
/// sc create myService binPath=<Path to your compiled executable>
/// ```
///
/// It is important that you provide the same service name to the macro (see below).
///
/// Once everything is set up, you can start and stop your service from the SCM by typing
/// `services.msc` into the Windows prompt; starting the EXE directly will have no effect since
/// the SCM will reject all attempts to register a ServiceMain function which it did not request.
///
/// # Examples
///
/// ```
/// #![no_main]
/// #![feature(link_args)]
/// #![link_args = "-Wl,--subsystem,windows"]
///
/// use std::os::raw::{c_char, c_int, c_void};
/// use std::sync::mpsc::Receiver;
///
/// #[macro_use]
/// extern crate winservice;
///
/// #[allow(non_snake_case)]
/// #[no_mangle]
/// pub extern "system" fn WinMain(hInstance : *const c_void, hPrevInstance : *const c_void,
///     lpCmdLine : *const c_char, nCmdShow : c_int) -> c_int
/// {
///     Service!("myService", service_main)
/// }
///
/// fn service_main(args : Vec<String>, end : Receiver<()>) -> u32 {
///     loop {
///         // Do some work
///         if let Ok(_) = end.try_recv() { break; } }
///     0 }
///
/// # fn main() {}
/// ```
///
/// # How it works
///
/// Since The ServiceCtrlDispatcher doesn't allow for a custom pointer to be passed to ServiceMain,
/// we cannot use a closure or any other means to obtain context information about the way we are
/// called. Thus the only option is to have a separate ServiceMain function for each call of
/// `Service!`. But since winservice will already be compiled when you want to create your service,
/// we have to do it here. The macro creates said ServiceMain function as a wrapper which calls
/// directly into the crate with your custom service_main and then feeds this wrapper to the
/// ServiceCtrlDispatcher.
#[macro_export]
macro_rules! Service { ( $name:expr, $function:ident ) => { {
    use std::os::raw::{c_char, c_int};
    extern "C" fn wrapper(argc : c_int, argv : *const *const c_char) {
        winservice::dispatch($name, $function, argc, argv); }
    return winservice::serve(wrapper); } } }

/// This should never be directly called from the user.
pub fn dispatch(name : &str, service_main : fn(Vec<String>, Receiver<()>) -> u32,
argc : c_int, argv : *const *const c_char) {
    let (mut status, rx) = SERVICE_STATUS::initialize(name);
    let args = (0..argc as isize).map(|i| unsafe { CStr::from_ptr(*argv.offset(i))
        .to_string_lossy().into_owned() }).collect::<Vec<String>>();
    status.exit_with(service_main(args, rx));
}

/// This should never be directly called from the user.
pub fn serve(wrapper: extern "C" fn(c_int, *const *const c_char)) -> i32 {
    let table = SERVICE_TABLE_ENTRY::with_wrapper(wrapper);
    unsafe { return StartServiceCtrlDispatcherA(&table as *const SERVICE_TABLE_ENTRY); }
}

