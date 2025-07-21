use std::{ffi::CStr, os::raw::c_void};

pub fn init_vulkan(hinstance: windows::Win32::Foundation::HINSTANCE, hwnd: windows::Win32::Foundation::HWND) {
    let entry = ash::Entry::linked();
    let app_info = ash::vk::ApplicationInfo::builder()
        .application_name(CStr::from_bytes_with_nul(b"RDelta\0").unwrap())
        .application_version(0)
        .engine_name(CStr::from_bytes_with_nul( b"Taylor\0").unwrap())
        .engine_version(0)
        .api_version(ash::vk::API_VERSION_1_3);

    let extension_names = vec![
        ash::extensions::khr::Surface::name().as_ptr(),
        ash::extensions::khr::Win32Surface::name().as_ptr()
    ];

    let instance_info = ash::vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_extension_names(&extension_names);

    unsafe {
        let instance = entry.create_instance(&instance_info, None).unwrap();

        let surface_loader = ash::extensions::khr::Win32Surface::new(&entry, &instance);
        let surface_info = ash::vk::Win32SurfaceCreateInfoKHR::builder()
            .hinstance(hinstance.0)
            .hwnd(hwnd.0);

        let surface = surface_loader
            .create_win32_surface(&surface_info, None)
            .expect("Failed to create Win32 surface");
    }
}