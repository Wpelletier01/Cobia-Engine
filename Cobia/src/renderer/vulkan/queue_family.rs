
// TODO: add comment

use ash::vk;


pub struct QueueFamilyIndices {

    graphicfamily: Option<u32>,
    presentfamily: Option<u32>,


}
//
impl QueueFamilyIndices {

    pub(crate) fn is_complete(&self) -> bool { self.graphicfamily.is_some() }

    pub(crate) fn get_graphic_index(&self) -> Option<u32> { self.graphicfamily }
    
    pub(crate) fn get_present_index(&self) -> Option<u32> { self.presentfamily }




}


pub(crate) fn find_queue_family(inst: &ash::Instance,pdevice: vk::PhysicalDevice) -> QueueFamilyIndices {

    let qfamilies  = unsafe {
        
        inst.get_physical_device_queue_family_properties(pdevice)

    };

    let mut qindice = QueueFamilyIndices { graphicfamily: None, presentfamily: None };

    let mut index = 0;
    for queue_family in qfamilies.iter() {
        
        if queue_family.queue_count > 0
            && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
        {
            qindice.graphicfamily = Some(index);
        }


        if qindice.is_complete() {
            break;
        }

        index += 1;
    }


    qindice


}
//
//
