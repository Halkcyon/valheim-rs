#![allow(clippy::ptr_arg)]

fn main() {
    windows::build!(windows::win32::system_services::GenerateConsoleCtrlEvent);
}
