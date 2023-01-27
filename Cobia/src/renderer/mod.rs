
pub(crate) mod vulkan;
pub(crate) mod primitives;
pub(crate) mod surface;


use crate::core::logs::{CVLK, CTRACE, CINFO, CDEBUGS, CWARN, CDEBUG, CFATAL};

use std::sync::Arc;

use surface::CSurface;
use vulkan::VlkBase;

use vulkano::{
    instance::{
        debug::ValidationFeatureEnable,
        Instance,
        InstanceCreateInfo,
        InstanceExtensions
    },
    VulkanLibrary,
    Version,
    sync::{
        self,
        NowFuture,
        GpuFuture,

    }

};

use error_stack::Result;

use crate::core::error_handler::{ERendering, EVlkApi};
use crate::define::ENGINE_VERSION;

//
pub(crate) struct RenderingSys {

    vlk_sys: VlkBase,
    surface: CSurface,
    recreate_sc: bool,
    previous_frame_end: Option<Box<dyn GpuFuture>>

}
//
impl RenderingSys {

    pub fn new(
        app_names:  &str,
        app_ver:    (u32,u32,u32),
    ) -> Result<Self,ERendering> {
        //
        let instance = create_vlk_instance(app_names,app_ver)
            .map_err(|e| e
                .change_context(ERendering::SYSTEM)
                .attach_printable("Initialisation failed")

            )?;

        CVLK("Successfully create Vulkan Instance");

        CTRACE("Start creating the CSurface with winit");

        let csurf = CSurface::new(instance.clone()).map_err( |e|
            e
                .change_context(ERendering::SYSTEM)
                .attach_printable("Initialisation failed")
            )?;

        CINFO("Successfully create Csurface struct");

        CTRACE("Start Vulkan base component creations");

        let vbase = VlkBase::init(
            &csurf,
            instance).map_err(|e| e
                .change_context(ERendering::SYSTEM)
                .attach_printable("Initialisation failed")
            )?;

        CINFO("Vulkan base initialisation done");

        let recreate_sc = false;
        let previous_frame_end = Some(sync::now(vbase.get_device()).boxed());

        Ok(
            RenderingSys {

                vlk_sys:        vbase,
                surface:        csurf,
                                recreate_sc,
                                previous_frame_end

            }
        )

    }
    //
    pub(crate) fn need_to_recreate_swapchain(&self) -> bool { self.recreate_sc }
    //
    pub(crate) fn set_window_title(&mut self, title:&str) -> Result<(),ERendering> {

        self.surface.set_title(title,self.vlk_sys.get_instance())?;
        Ok(())
    }
    //
    pub(crate) fn set_window_size(&mut self,w:u16,h:u16) -> Result<(),ERendering> {

        self.surface.set_win_size(w,h,self.vlk_sys.get_instance())?;
        Ok(())
    }
    //
}
//
//
fn create_vlk_instance(app_name:&str,app_ver:(u32,u32,u32)) -> Result<Arc<Instance>,EVlkApi> {


    let lib = VulkanLibrary::new().map_err(|e|
        EVlkApi::LIBRARY
            .attach_printable_default(e)
    )?;

    // TODO: check for other important extension and find solution/compromise when some are
    // TODO: unavailable

    let available_ext = lib.supported_extensions();

    let mut req_extension = vulkano_win::required_extensions(&lib);

    if !available_ext.ext_debug_utils {

        CFATAL("ext_debug_utils is unsupported on this machine");
        return Err(EVlkApi::INSTANCE
            .as_report()
            .attach_printable("ext_validation_features unsupported")
        );

    } else {

        CDEBUG("debug_utils extension supported");

        req_extension.ext_debug_utils = true;

    }

    if !available_ext.ext_validation_features {

        CWARN("ext_validation_features is not supported on this machine");



    } else {

        CDEBUG("ext_validation_features supported");
        req_extension.ext_validation_features = true


    }


    let layers = vec!["VK_LAYER_KHRONOS_validation".to_owned()];



    let inst_info = InstanceCreateInfo{

        application_name:           Some(app_name.to_owned()),
        application_version:        Version{
            major: app_ver.0,
            minor: app_ver.1,
            patch: app_ver.2
        },
        engine_name:            Some("Cobia".to_owned()),
        engine_version:         Version{
            major: ENGINE_VERSION.0,
            minor: ENGINE_VERSION.1,
            patch: ENGINE_VERSION.2

        },
        enabled_extensions:     req_extension,
        enabled_layers:         vec![
            "VK_LAYER_KHRONOS_validation".to_string()
        ],
        enumerate_portability:  true,
        enabled_validation_features: if req_extension.ext_validation_features {
            vec![
                ValidationFeatureEnable::BestPractices,
                ValidationFeatureEnable::DebugPrintf,
                ValidationFeatureEnable::SynchronizationValidation,
            ]
        } else {
            vec![]

        },
        ..Default::default()

    };

    let inst = Instance::new(lib,inst_info).map_err(|e|
        EVlkApi::INSTANCE
            .attach_printable_default(e)
    )?;

    Ok(inst)

}
//
fn validate_available_extension(available_ext:InstanceExtensions) {


    if !available_ext.ext_debug_utils {

        CWARN("debug_utils extension is not supported on this machine")

    }





}
