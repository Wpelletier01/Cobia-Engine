
pub mod WindowEvent;

use crate::core::error_handler::ERendering;

use winit::window::WindowBuilder;
use winit::event_loop::EventLoop;
use winit::dpi::LogicalSize;
use vulkano::instance::Instance;
use error_stack::Result;

use std::sync::Arc;
use vulkano::swapchain::Surface;
use vulkano_win::VkSurfaceBuild;

pub(crate) struct CSurface {

    surf:  Arc<Surface>,
    eloop: EventLoop<()>

}
//
impl CSurface {

    pub(crate) fn new(
        title:  &str,
        width:  u16,
        height: u16,
        inst:   Arc<Instance>) -> Result<Self,ERendering> {


        let eloop = EventLoop::new();

        let win = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(width,height))
            .build_vk_surface(&eloop,inst)
            .map_err(|e|ERendering::SURFACE
                .as_report()
                .attach_printable(format!("{}",e))
            )?;


        Ok(
            Self {
                surf:   win,
                eloop
            }
        )

    }
    //
    pub(crate) fn get_surface(&self) -> &Arc<Surface> { &self.surf }


}