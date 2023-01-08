#![allow(dead_code)]


pub(crate) mod utils;
pub(crate) mod queue_family;
pub(crate) mod debug;
pub(crate) mod physical_device;
pub(crate) mod logical_device;
pub(crate) mod validation_layer;
pub(crate) mod surface;
pub(crate) mod swapchain;
pub(crate) mod instance;
pub(crate) mod graphic_pipeline;
 

use error_stack::{Result, ResultExt};

use crate::core::error_handler::{EVlkApi,ERendering};


const REQUIRED_LAYERS: [&str;1] = ["VK_LAYER_KHRONOS_validation"];

pub(crate) struct VlkSystem {

    entry:              ash::Entry,
    instance:           ash::Instance,
    vlayer:             validation_layer::ValidationLayer,
    debuger:            debug::VlkDebugSys,
    pdevices:           Vec<ash::vk::PhysicalDevice>,
    logical_device:     logical_device::LogicDevice,
    surface:            surface::VSurface,
    graphics_pipeline:  graphic_pipeline::GPipeline

}
//
impl VlkSystem {


    fn init(
        appname: &str,
        window:&winit::window::Window,
        shaders: Vec<&str>

    ) -> Result<Self,ERendering> {


        let entry = unsafe {
            match ash::Entry::load() {

                Ok(entry_) => entry_,
                Err(e) => return Err(
                    EVlkApi::ENTRY
                        .attach_printable_default(e)
                        .change_context(ERendering::VLK)
                        .attach_printable("Cannot initialize the vulkan system")
                    )


            }
        }; 

        let vlayer = validation_layer::ValidationLayer::new(REQUIRED_LAYERS);

        vlayer.check(&entry);


        let instance = instance::create_instance(appname, &entry)
            .change_context(ERendering::VLK)
            .attach_printable("Can't initialize the vulkan System")?;
        

        let vdebuger = debug::set_debug_utils(&entry, &instance)
            .change_context(ERendering::VLK)
            .attach_printable("Can't initialize the vulkan System")?;

        
        let pdevices = physical_device::get_physical_devices(&instance)
            .change_context(ERendering::VLK)
            .attach_printable("Can't initialize the vulkan System")?;

        // TODO: for now we just take the first pdevice in the vector 
        // TODO: implement something later
        let logic_device =  logical_device::LogicDevice::new(&instance, pdevices[0], &vlayer)
            .change_context(ERendering::VLK)
            .attach_printable("Can't initialize the vulkan System")?;

        let vsurface = surface::VSurface::new(&instance, &entry, window)
            .change_context(ERendering::VLK)
            .attach_printable("Can't initialize the vulkan System")?;


        

        let gpipeline = graphic_pipeline::GPipeline::new(dev, sc_extent, shaders, surface_format)

        Ok(
            VlkSystem{
                
                entry:          entry,
                vlayer:         vlayer,
                instance:       instance,
                debuger:        vdebuger,
                pdevices:       pdevices,
                logical_device: logic_device,
                surface:        vsurface

            }
        )


    }
    //
    //

    
}

impl Drop for VlkSystem {


    fn drop(&mut self) {

        unsafe {
            
      
            

        }
         
    }

}
//
