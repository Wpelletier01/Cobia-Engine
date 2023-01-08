
// TODO: add comment



use super::{EVlkApi,Result,surface::VSurface,queue_family::QueueFamilyIndices};

use std::ptr;

use ash::vk;
use error_stack::ResultExt;
use num::clamp;

struct VSwapChainInfo {

    capabilities:  vk::SurfaceCapabilitiesKHR,
    formats:       Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,
    

}
//
impl VSwapChainInfo {
    
    fn new(pdevice: vk::PhysicalDevice, surface: &VSurface) -> Result<Self,EVlkApi> {

        let capabilities = surface.get_pdev_surface_capabilities(pdevice)
            .change_context(EVlkApi::SWAPCHAIN)
            .attach_printable("Can't access physical surface capabilities for swapchain info")?;

        let formats =  surface.get_pdev_surface_format(pdevice)
            .change_context(EVlkApi::SWAPCHAIN)
            .attach_printable("Can't access physical surface format for swapchain info")?;

        let present_modes = surface.get_pdev_surface_present_mode(pdevice)
            .change_context(EVlkApi::SWAPCHAIN)
            .attach_printable("Can't access physical surface present modes for swapchain info")?;


        Ok(VSwapChainInfo { 

            capabilities: capabilities,
            formats: formats,
            present_modes: present_modes 
            }
        )


    }

    fn get_available_formats(&self) -> &Vec<vk::SurfaceFormatKHR> { &self.formats }

    fn get_available_present_modes(&self) -> &Vec<vk::PresentModeKHR> { &self.present_modes }

    fn get_capabilities(&self) -> &vk::SurfaceCapabilitiesKHR { &self.capabilities }

}



pub(crate) struct VSwapChain {

    loader:     ash::extensions::khr::Swapchain,
    swapchain:  vk::SwapchainKHR,
    image:      Vec<vk::Image>,
    image_view: Vec<vk::ImageView>,
    format:     vk::Format,
    extent:     vk::Extent2D,

    info:       VSwapChainInfo

}
//
impl VSwapChain {
    
    pub(crate) fn new(
        widht:          u32,
        height:         u32,
        inst:           &ash::Instance,
        dev:            &ash::Device,
        pdev:           vk::PhysicalDevice,
        surface:        &VSurface,
        qfamilyIndice:  &QueueFamilyIndices) -> Result<Self,EVlkApi> {
    
        let info = VSwapChainInfo::new(pdev, surface)
            .change_context(EVlkApi::SWAPCHAIN)
            .attach_printable("Cant get info for swapchain creation")?;

        let findex = Self::choose_format(info.get_available_formats());

        let format = info.get_available_formats()[findex];
    
        let pindex = Self::choose_present_mode(info.get_available_present_modes());

        let present_mode = info.get_available_present_modes()[pindex];

        let extent = Self::choose_extent(widht, height,info.get_capabilities());

        // it's recommended to use one image more than the minimum but also we need to make sure 
        // that it exceeds the maximum supported

        let mut image_count = info.get_capabilities().min_image_count + 1;

        if info.get_capabilities().max_image_count > 0 && image_count > info.get_capabilities().max_image_count {

            image_count = info.get_capabilities().max_image_count;
        }

        // 

        let (image_sharing_mode, queue_family_index_count, queue_family_indices)  =
            if qfamilyIndice.get_graphic_index() == qfamilyIndice.get_present_index() {
                (

                    vk::SharingMode::CONCURRENT,
                    1,
                    vec![
                        qfamilyIndice.get_graphic_index().unwrap(),
                        qfamilyIndice.get_present_index().unwrap()
                    ]
                )
            } else {
                (vk::SharingMode::EXCLUSIVE, 0, vec![])  
        }; 

        let create_info = vk::SwapchainCreateInfoKHR {

            s_type:                 vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next:                 ptr::null(),
            flags:                  vk::SwapchainCreateFlagsKHR::empty(),
            surface:                surface.get_surface(),
            min_image_count:        image_count,
            image_color_space:      format.color_space,
            image_format:           format.format,
            image_extent:           extent,
            image_usage:            vk::ImageUsageFlags::COLOR_ATTACHMENT,
                                    image_sharing_mode,
            p_queue_family_indices: queue_family_indices.as_ptr(),
                                    queue_family_index_count,
            pre_transform:          info.capabilities.current_transform,
            composite_alpha:        vk::CompositeAlphaFlagsKHR::OPAQUE,
                                    present_mode,
            clipped:                vk::TRUE,
            old_swapchain:          vk::SwapchainKHR::null(),
            image_array_layers:     1

        };


        let loader = ash::extensions::khr::Swapchain::new(inst, dev);

        let sc = unsafe {
            match loader.create_swapchain(&create_info, None) {

                Ok(s) => s,
                Err(e) => return Err(EVlkApi::SWAPCHAIN.attach_printable_default(e))

            }
        };

        let sc_image = unsafe {
            
            match loader.get_swapchain_images(sc) {

                Ok(img) => img,
                Err(e) => return Err(EVlkApi::SWAPCHAIN.attach_printable_default(e))

            }

        };

        let sc_img_view = Self::create_image_views(dev, format.format, &sc_image)?;

        Ok(Self{ 

            loader:     loader,
            swapchain:  sc,
            format:     format.format,
            extent:     extent,
            image:      sc_image,
            image_view: sc_img_view,
            info:       info
            
            }
        )

    
    }


    fn choose_format(available_formats:&Vec<vk::SurfaceFormatKHR>) -> usize {


        let mut index:usize = 0;

        for format in available_formats.iter() {


            if format.format == vk::Format::B8G8R8A8_SRGB && 
                format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR {

                return index;


            }

            index += 1;


        }

        0

    }

    fn choose_present_mode(available_present_modes:&Vec<vk::PresentModeKHR>) -> usize {


        let mut index:usize = 0;

        for present_mode in available_present_modes.iter() {

            if present_mode == &vk::PresentModeKHR::MAILBOX {

                return index;

            }

            index += 1;

        }

        0

    }

    fn choose_extent(w:u32,h:u32,capability: &vk::SurfaceCapabilitiesKHR) -> vk::Extent2D {


        if capability.current_extent.width != u32::max_value() {

            capability.current_extent

        } else {

            vk::Extent2D {

                width: clamp(
                    w, 
                    capability.min_image_extent.width, 
                    capability.max_image_extent.width
                ),
                height: clamp(
                    h,
                    capability.min_image_extent.height,
                    capability.max_image_extent.height
                )


            }

        }


    }

    
    fn create_image_views(

        dev:            &ash::Device,
        surface_format: vk::Format,
        images:         &Vec<vk::Image> ) -> Result<Vec<vk::ImageView>,EVlkApi> {

            
        let mut sc_img_view:Vec<vk::ImageView> = Vec::new();
        
        for img in images.iter() {

            let create_info = vk::ImageViewCreateInfo {

                s_type:      vk::StructureType::IMAGE_VIEW_CREATE_INFO,
                p_next:      ptr::null(),
                flags:       vk::ImageViewCreateFlags::empty(),
                view_type:   vk::ImageViewType::TYPE_2D,
                format:      surface_format,
                components:  vk::ComponentMapping {

                    r: vk::ComponentSwizzle::IDENTITY,
                    g: vk::ComponentSwizzle::IDENTITY,
                    b: vk::ComponentSwizzle::IDENTITY,
                    a: vk::ComponentSwizzle::IDENTITY

                },

                subresource_range: vk::ImageSubresourceRange {

                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1

                },
                image: *img

            };

            let img_view = unsafe {
                
                match dev.create_image_view(&create_info, None) {

                    Ok(i_v) => i_v,
                    Err(e) => return Err(
                        EVlkApi::SWAPCHAIN
                            .as_report()
                            .attach_printable(format!(
                                "unable to create an image view caused by: {}",
                                e.to_string()
                                )
                            )

                        )

                }

            };

            sc_img_view.push(img_view);

        }


        Ok(sc_img_view)

    }


}

