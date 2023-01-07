

use super::{EVlk, queue_family,validation_layer};

use std::ptr;

use ash::vk;

pub(crate) struct LogicDevice {

    device: ash::Device,
    queue:  vk::Queue,

} 
//
impl LogicDevice {


    pub(crate) fn new(
        inst:       &ash::Instance,
        pdevice:    vk::PhysicalDevice,
        vlayer:     &validation_layer::ValidationLayer ) -> Result<Self,EVlk> {

        let indices = queue_family::find_queue_family(inst, pdevice);

        let queue_priorities = [1.0_f32];

        let queue_create_info = vk::DeviceQueueCreateInfo {

            s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
            p_next: ptr::null(),
            flags:  vk::DeviceQueueCreateFlags::empty(),
            queue_family_index: indices.get_graphic_index().unwrap(),
            p_queue_priorities: queue_priorities.as_ptr(),
            queue_count: queue_priorities.len() as u32,

        };

        let physical_device_features = vk::PhysicalDeviceFeatures {
            ..Default::default() // default just enable no feature.
        };


        let enable_layer_names = vlayer.get_enable_layer();

        let dev_create_info = vk::DeviceCreateInfo {

            s_type: vk::StructureType::DEVICE_CREATE_INFO,
            p_next: ptr::null(),
            flags:  vk::DeviceCreateFlags::empty(),
            queue_create_info_count: 1,
            p_queue_create_infos: &queue_create_info,
            enabled_layer_count: if vlayer.is_enabled() {
                enable_layer_names.len() 
            } else { 
                0 
            } as u32,
            pp_enabled_layer_names: if vlayer.is_enabled() {

                enable_layer_names.as_ptr()
                
            } else {

                ptr::null()

            },
            enabled_extension_count: 0,
            pp_enabled_extension_names: ptr::null(),
            p_enabled_features: &physical_device_features


        };

        let device: ash::Device = unsafe {
            
            match inst.create_device(pdevice , &dev_create_info, None) {

                Ok(d) => d,
                Err(e) => {

                    return Err(EVlk::LOGIC_DEVICE(e.to_string()));
                }

            }

        };

        let gqueue = unsafe {

            device.get_device_queue(indices.get_graphic_index().unwrap(), 0)

        };

        Ok(LogicDevice{ device: device, queue: gqueue })

    }



}


