
use winapi::um::winuser::{MessageBoxW, MB_OK};
use std::ptr;

fn main() {
    unsafe {
        MessageBoxW(
            ptr::null_mut(),    // hWnd: HWND
            encode("お疲れ様です。世界").as_ptr(),    // lpText: LPCWSTR
            encode("Hello, world.").as_ptr(),   // lpCaption: LPCWSTR
            MB_OK   // uType: UINT
        );
    }
}

fn encode(source: &str) -> Vec<u16> {
    source.encode_utf16().chain(Some(0)).collect()
}
