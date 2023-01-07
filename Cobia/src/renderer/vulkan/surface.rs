
use ash::vk;

use super::EVlk;

use std::ptr;

pub(crate) struct VSurface {

    surface: vk::SurfaceKHR,
    loader:  ash::extensions::khr::Surface,


}
//
impl VSurface {

    pub(crate) fn get_pdev_surface_capabilities(&self,pdev:vk::PhysicalDevice) -> Result<vk::SurfaceCapabilitiesKHR,EVlk> {

        let cap = unsafe {
            match self.loader.get_physical_device_surface_capabilities(pdev, self.surface) {

                Ok(caps) => caps,
                Err(e) => return Err(EVlk::SURFACE_DATA(e.to_string()))
            }

        };

    
        Ok(cap)

    }

    pub(crate) fn get_pdev_surface_format(&self,pdev:vk::PhysicalDevice) 
        -> Result<Vec<vk::SurfaceFormatKHR>,EVlk> {

        let format = unsafe {

            match self.loader.get_physical_device_surface_formats(pdev,self.surface) {

                Ok(formats) => formats,
                Err(e) => return Err(EVlk::SURFACE_DATA(e.to_string()))

            }


        };

        Ok(format)

    }

    pub(crate) fn get_pdev_surface_present_mode(&self,pdev:vk::PhysicalDevice) 
        -> Result<Vec<vk::PresentModeKHR>,EVlk> {

        let present_modes = unsafe {
            match self.loader.get_physical_device_surface_present_modes(pdev,self.surface) {
                Ok(ps) => ps,
                Err(e) => return Err(EVlk::SURFACE_DATA(e.to_string()))
            
            }

        };


        Ok(present_modes)


    }
    
    pub(crate) fn get_surface(&self) -> vk::SurfaceKHR { self.surface }

    

}


//
//
#[cfg(target_os = "linux")]
pub(crate) fn create_window_surface(
    inst:   &ash::Instance,
    entry:  &ash::Entry,
    window: &winit::window::Window ) -> Result<VSurface,EVlk> {

    use winit::platform::unix::WindowExtUnix;
    use ash::extensions::khr::XlibSurface;

    let x11_display = match window.xlib_display() {

        Some(d) => d,
        None => return Err(EVlk::SURFACE("Current display session doesn't use xlib".to_string()))

    };

    let x11_window = match window.xlib_window() {

        Some(d) => d,
        None => return Err(EVlk::SURFACE("Current display session doesn't use xlib".to_string()))

    };


    let x11_create_info = vk::XlibSurfaceCreateInfoKHR {

        s_type: vk::StructureType::XLIB_SURFACE_CREATE_INFO_KHR,
        p_next: ptr::null(),
        flags:  Default::default(),
        window: x11_window as vk::Window,
        dpy: x11_display as *mut vk::Display

    };

    let xlib_surf_loader = XlibSurface::new(entry, inst);

    let surf = unsafe { 
        match xlib_surf_loader.create_xlib_surface(&x11_create_info, None) {

            Ok(s) => s,
            Err(e) => return Err(EVlk::SURFACE(e.to_string()))

        }
    
    };

    let surf_loader = ash::extensions::khr::Surface::new(entry, inst);


    Ok(VSurface { surface: surf, loader: surf_loader} ) 

}