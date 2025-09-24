use std::{ffi::CStr, os::raw::c_void};

pub fn init_vulkan(hinstance: windows::Win32::Foundation::HINSTANCE, hwnd: windows::Win32::Foundation::HWND) {
    let entry = ash::Entry::linked();
    let app_info = ash::vk::ApplicationInfo {
        p_application_name: std::ffi::CString::new("Physics engine").unwrap().as_ptr(),
        application_version: 0,
        p_engine_name: std::ffi::CString::new("Physics engine").unwrap().as_ptr(),
        engine_version: 0,
        api_version: ash::vk::API_VERSION_1_3,
        ..Default::default()
    };

    // let app_info = ash::vk::ApplicationInfo::builder()
    //     .application_name(CStr::from_bytes_with_nul(b"RDelta\0").unwrap())
    //     .application_version(0)
    //     .engine_name(CStr::from_bytes_with_nul( b"Taylor\0").unwrap())
    //     .engine_version(0)
    //     .api_version(ash::vk::API_VERSION_1_3);

    let extension_names = vec![
        ash::khr::surface::NAME.as_ptr(),
        ash::khr::win32_surface::NAME.as_ptr()
        //ash::extensions::khr::Surface::name().as_ptr(),
        //ash::extensions::khr::Win32Surface::name().as_ptr()
        ];

    let instance_info = ash::vk::InstanceCreateInfo {
        p_application_info: &app_info,
        pp_enabled_extension_names: extension_names.as_ptr(),
        enabled_extension_count: extension_names.len() as u32,
        ..Default::default()
    };

    // let instance_info = ash::vk::InstanceCreateInfo::builder()
    //     .application_info(&app_info)
    //     .enabled_extension_names(&extension_names);

    unsafe {
        let instance = entry.create_instance(&instance_info, None).unwrap();

        let surface_loader = ash::khr::win32_surface::Instance::new(&entry, &instance);
        //let surface_loader = ash::extensions::khr::Win32Surface::new(&entry, &instance);
        let surface_info = ash::vk::Win32SurfaceCreateInfoKHR {
            hinstance: hinstance.0 as isize,
            hwnd: hwnd.0 as isize,
            ..Default::default()
        };
        // let surface_info = ash::vk::Win32SurfaceCreateInfoKHR::builder()
        //     .hinstance(hinstance.0)
        //     .hwnd(hwnd.0);

        let surface = surface_loader
            .create_win32_surface(&surface_info, None)
            .expect("Failed to create Win32 surface");
    }
}