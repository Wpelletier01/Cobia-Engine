// TODO: add comment

use crate::core::logs::{CDEBUG, CDEBUGS, CINFO, CTRACE};
use crate::core::error_handler::ERendering;

use winit::window::{Window, WindowBuilder};
use winit::event_loop::EventLoop;
use winit::dpi::LogicalSize;
use vulkano::instance::Instance;
use error_stack::Result;

use std::sync::Arc;
use vulkano::swapchain::Surface;
use vulkano_win::VkSurfaceBuild;
//
//
//
// ------------------------------------------------------------------------------------------------
// Const
//
// Default window properties
const DEFAULT_WIDTH:u16 = 1000;
const DEFAULT_HEIGHT:u16 = 1000;
const DEFAULT_TITLE: &str = "Cobia Engine";
//
//
/// Store important stuff for the display. Help to link winit Window with Vulkan, get interesting
/// event that winit captured
pub(crate) struct CSurface {

    surf:       Arc<Surface>,
    eloop:      EventLoop<()>,
    title:      String,
    width:      u16,
    height:     u16
}
//
impl CSurface {
    //
    /// Initialise the field with default value
    ///
    /// # Parameters
    ///
    /// * 'inst' - A Vulkan instance to be link to the window surface
    ///
    pub(crate) fn new(inst: Arc<Instance>) -> Result<Self,ERendering> {
        
        let eloop = EventLoop::new();

        let winit_win = WindowBuilder::new()
            .with_title(DEFAULT_TITLE)
            .with_inner_size(LogicalSize::new(DEFAULT_WIDTH,DEFAULT_HEIGHT));

        CDEBUGS(
            "Create window builder with default value. Title: {} width: {} height: {}",
            &[DEFAULT_TITLE,&DEFAULT_WIDTH.to_string(),&DEFAULT_HEIGHT.to_string()]
        );


        let win =  winit_win
            .build_vk_surface(&eloop,inst)
            .map_err(|e|ERendering::SURFACE
                .as_report()
                .attach_printable(format!("{}",e))
            )?;
        
        CINFO("Successfully create a vulkan surface with a winit Window");
        
        Ok(
            Self {
                surf:   win,
                eloop,
                title: DEFAULT_TITLE.to_string(),
                width: DEFAULT_WIDTH,
                height: DEFAULT_HEIGHT
            }
        )

    }
    //
    pub(crate) fn get_surface(&self) -> &Arc<Surface> { &self.surf }
    //
    fn change_vk_surface(&mut self,instance:Arc<Instance>,win_builder:WindowBuilder) -> Result<(),ERendering> {

        self.surf = win_builder
            .build_vk_surface(&self.eloop,instance)
            .map_err(|e|ERendering::SURFACE
                .as_report()
                .attach_printable(format!("{}",e))
            )?;

        Ok(())

    }

    //
    pub(crate) fn set_title(&mut self,title:&str,instance:Arc<Instance>) -> Result<(),ERendering> {

        self.title = title.to_string();

        let winit_win = WindowBuilder::new()
            .with_title(&self.title)
            .with_inner_size(LogicalSize::new(self.width,self.height));

        self.change_vk_surface(instance,winit_win)?;

        Ok(())


    }
    //
    pub(crate) fn set_win_size(&mut self,w:u16,h:u16,instance:Arc<Instance>) -> Result<(),ERendering> {

        self.width = w;
        self.height = h;

        let winit_win = WindowBuilder::new()
            .with_title(&self.title)
            .with_inner_size(LogicalSize::new(self.width,self.height));


        self.change_vk_surface(instance,winit_win)?;

        Ok(())
    }

}