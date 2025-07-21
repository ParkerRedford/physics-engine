use windows::{
    core::*, Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        System::LibraryLoader::*,
        UI::WindowsAndMessaging::*
    }
};

use windows::Win32::Foundation::{HINSTANCE, HWND};

pub fn init_windows() -> (HINSTANCE, HWND) {
    extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            match message {
                WM_PAINT => {
                    LRESULT(0)
                }
                WM_SIZE => {
                    LRESULT(0)
                }
                WM_DESTROY => {
                    PostQuitMessage(0);
                    LRESULT(0)
                }
                _ => DefWindowProcA(window, message, wparam, lparam)
            }
        }
    }

    unsafe {
        let hmodule = GetModuleHandleA(None).expect("Failed to get module handle");
        let hinstance = Some(HINSTANCE(hmodule.0));

        let windows_class = s!("windows");

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW).expect(""),
            hInstance: hinstance.unwrap(),
            lpszClassName: windows_class,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: HICON::default(),
            hbrBackground: CreateSolidBrush( COLORREF(COLOR_WINDOW.0 as u32)),
            lpszMenuName: PCSTR::null()
        };

        RegisterClassA(&wc);
        
        let hwnd = CreateWindowExA(
            WINDOW_EX_STYLE(0),
            windows_class,
            s!("RDelta"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            hinstance,
            None
        ).expect("Failed to create window");
        
        (hinstance.unwrap(), hwnd)
    }
}

pub fn run_windows_loop() {
    let mut message = MSG::default();

    unsafe {
        while GetMessageA(&mut message, None, 0, 0) != false {
            TranslateMessage(&message);
            DispatchMessageA(&message);
        }
    }
}