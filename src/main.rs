mod vulkan;
mod math;

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::{HWND, HINSTANCE};
pub mod vk_windows;

pub mod expr;
use expr::*;

use vk_windows::*;
use vulkan::*;

fn main() {
    let expr = add(vec![
        c(3), c(9), c(4),
        mul(vec![c(2), c(3), c(4), c(5)]),
        c(5 + 9), c(7), mul(vec![c(6), c(2)])
    ]);

    // let expr = sub(vec![
    //     mul(vec![c(2), c(3), c(4), c(5)]),
    //     c(3), c(9), c(4)
    // ]);

    println!("{}", expr);
    println!("{}", expr.eval());

    let (hinstance, hwnd) = init_windows();

    #[cfg(target_os = "windows")]
    run_windows_loop();

    init_vulkan(hinstance, hwnd);
}
