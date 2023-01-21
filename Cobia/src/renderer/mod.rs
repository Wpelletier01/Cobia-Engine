
pub(crate) mod vulkan;
pub(crate) mod primitives;
pub(crate) mod surface;


//
use std::sync::Arc;

use surface::CSurface;
use vulkan::VlkBase;

use vulkano::{
    instance::{
        debug::ValidationFeatureEnable,
        Instance,
        InstanceCreateInfo
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
use crate::define::{ENGINE_VERSION,REQUIRED_LAYER};

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
        widht:      u16,
        height:     u16,

    ) -> Result<Self,ERendering> {

        let instance = create_vlk_instance(app_names,app_ver)
            .map_err(|e| e
                .change_context(ERendering::SYSTEM)
                .attach_printable("Initialisation failed")

            )?;

        let csurf = CSurface::new(
            app_names,
            widht,
            height,
            instance.clone())
            .map_err( |e| e
                .change_context(ERendering::SYSTEM)
                .attach_printable("Initialisation failed")

            )?;

        let vbase = VlkBase::init(
            app_names,
            app_ver,
            &csurf,
            instance).map_err(|e| e
                .change_context(ERendering::SYSTEM)
                .attach_printable("Initialisation failed")
            )?;

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

}
//
//
fn create_vlk_instance(app_name:&str,app_ver:(u32,u32,u32)) -> Result<Arc<Instance>,EVlkApi> {

    let lib = VulkanLibrary::new().map_err(|e|
        EVlkApi::LIBRARY
            .attach_printable_default(e)
    )?;

    let mut req_extension = vulkano_win::required_extensions(&lib);

    req_extension.ext_debug_utils = true;

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
        enabled_layers:         REQUIRED_LAYER,
        enumerate_portability:  true,
        enabled_validation_features: vec![

            ValidationFeatureEnable::BestPractices,
            ValidationFeatureEnable::DebugPrintf,
            ValidationFeatureEnable::SynchronizationValidation

        ],
        ..Default::default()

    };

    let inst = Instance::new(lib,inst_info).map_err(|e|
        EVlkApi::INSTANCE
            .attach_printable_default(e)
    )?;

    Ok(inst)

}
