mod vulkan;
mod math;

use windows::Win32::Foundation::{HWND, HINSTANCE};

pub mod vk_windows;
pub mod vk_linux;
pub mod vk_macos;

pub mod expr;
use expr::*;

use math::constants::*;

use vk_windows::*;
use vk_linux::*;
use vk_macos::*;

use vulkan::*;

fn main() {
    let expr = add(vec![
        c(3), c(9), c(4),
        mul(vec![c(2), c(3), c(4), c(5)]),
        c(5 + 9), c(7), mul(vec![c(6), c(2)])
    ]);

    let n = constants::factorial(0);
    let consts = constants::Constants::new();

    println!("Factorial: {}", n);
    println!("Euler's constant: {}", consts.E);

    // let expr = sub(vec![
    //     mul(vec![c(2), c(3), c(4), c(5)]),
    //     c(3), c(9), c(4)
    // ]);

    println!("{}", expr);
    println!("{}", expr.eval());

    let (hinstance, hwnd) = init_windows();

    #[cfg(target_os = "windows")]
    //run_windows_loop();

    #[cfg(target_os = "linux")]
    run_linux_loop();

    #[cfg(target_os = "macos")]
    run_macos_loop();

    init_vulkan(hinstance, hwnd);
}
