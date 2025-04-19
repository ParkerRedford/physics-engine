use std::cell::UnsafeCell;
use std::sync::OnceLock;

use windows::Win32::Graphics::OpenGL::ChoosePixelFormat;
use windows::{
    core::*, Win32::{
        Foundation::*,
        Graphics::{self, Gdi::*},
        System::LibraryLoader::*,
        UI::WindowsAndMessaging::*
    }
};

use windows::Win32::Graphics::OpenGL::*;
static mut HDC_GLOBAL: Option<HDC> = None;

fn main() {
    fn setup_opengl(hwnd: HWND) {
        unsafe {
            let hdc = GetDC(Some(hwnd));
            
            let pfd = PIXELFORMATDESCRIPTOR {
                nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16,
                nVersion: 1,
                dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
                iPixelType: PFD_TYPE_RGBA,
                cColorBits: 32,
                cDepthBits: 24,
                iLayerType: PFD_MAIN_PLANE.0 as u8,
                ..std::mem::zeroed()
            };
    
            let format = ChoosePixelFormat(hdc, &pfd);
            SetPixelFormat(hdc, format, &pfd);
    
            let hglrc = wglCreateContext(hdc);
            wglMakeCurrent(hdc, hglrc.unwrap());

            HDC_GLOBAL = Some(hdc);
        }
    }
    extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            match message {
                WM_PAINT => {
                    if let Some(hdc) = HDC_GLOBAL {
                        glClearColor(0.0, 0.0, 0.5, 0.0);
                        glClear(GL_COLOR_BUFFER_BIT);
                        SwapBuffers(hdc);
                    }
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
        let module = GetModuleHandleA(None).unwrap();
        let instance = Some(HINSTANCE(module.0));
        debug_assert!(instance.unwrap() != HINSTANCE(std::ptr::null_mut()), "Failed to get module handle");

        let windows_class = s!("windows");

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW).expect(""),
            hInstance: instance.unwrap(),
            lpszClassName: windows_class,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: HICON::default(),
            hbrBackground: CreateSolidBrush( COLORREF(COLOR_WINDOW.0 as u32)),
            lpszMenuName: PCSTR::null()
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);
        
        let hwnd = CreateWindowExA(
            WINDOW_EX_STYLE(0),
            windows_class,
            s!("This is a simple window"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            instance,
            None
        );

        setup_opengl(hwnd.unwrap());

        let mut message = std::mem::zeroed();

        while GetMessageA(&mut message, None, 0, 0) != false {
            TranslateMessage(&message);
            DispatchMessageA(&message);
        }
    }
}
