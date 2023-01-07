
use ash::vk;

use crate::core::logs::CDEBUGS;


use super::{utils,queue_family};
use super::EVlk;


//
//
pub(crate) fn get_physical_devices(inst:&ash::Instance) -> Result<Vec<vk::PhysicalDevice>,EVlk> {

    let devs = unsafe {
        
        match inst.enumerate_physical_devices() {

            Ok(d) => d,
            Err(e) => return Err(EVlk::PHYSICAL_DEVICES_ENUM(e.to_string()))

        }
        
    };

    let mut suitable_pdevice:Vec<vk::PhysicalDevice> = Vec::new();


    for dev in devs.iter() {

        if is_pdevice_suitable(inst,*dev) {

            suitable_pdevice.push(*dev);

        }

    }

    if suitable_pdevice.is_empty() {

        return Err(EVlk::PHYSICAL_DEVICES_FOUND);

    }

    Ok(suitable_pdevice)

}
//
//
fn is_pdevice_suitable(inst:&ash::Instance,pdevice:vk::PhysicalDevice) -> bool {


    let dev_properties = unsafe { inst.get_physical_device_properties(pdevice)};
    let dev_features = unsafe { inst.get_physical_device_features(pdevice) };
    
    pdevice_debug_info(&dev_properties);
    
    let qfamily = queue_family::find_queue_family(inst, pdevice);
    
    // TODO: check the type of device 

    qfamily.is_complete()

}
//
//
fn pdevice_debug_info(properties: &vk::PhysicalDeviceProperties) {

    let dev_name = utils::vk_to_string(&properties.device_name);

    let dev_id = properties.device_id.to_string();
        
    let driver_ver = properties.driver_version.to_string();


    CDEBUGS(
        "Physical Device: {}\n\tid: {}\n\tdriver version: {}",
        &[
            dev_name,
            &dev_id,
            &driver_ver
        ]
    
    );


}
//
//
