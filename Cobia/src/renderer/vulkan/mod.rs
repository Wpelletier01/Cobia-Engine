
pub(crate) mod debug;

use std::borrow::ToOwned;
use std::sync::Arc;

use vulkano::{
    Version,
    VulkanLibrary,
    instance::{
        Instance,
        InstanceCreateInfo,
        debug::{
            ValidationFeatureEnable,
            DebugUtilsMessenger
        }
    },
    device::{
        DeviceExtensions,
        DeviceCreateInfo,
        physical::{
            PhysicalDeviceType,
            PhysicalDevice

        },
        QueueFlags,
        Queue,
        QueueCreateInfo,
        Device,
        Features
    },
    swapchain::{
        acquire_next_image,
        AcquireError,
        Swapchain,
        SwapchainCreateInfo,
        SwapchainCreationError,
        SwapchainPresentInfo,
        Surface
    },
    image::{
        ImageUsage,
        SwapchainImage,
        view::{
            ImageView,
            ImageViewCreateInfo
        },
        traits::ImageAccess

    },
    shader::{
        ShaderModule
    },
    pipeline::{
        graphics::{
            GraphicsPipeline,
            render_pass::{
                PipelineRenderingCreateInfo
            },
            input_assembly::InputAssemblyState,
            vertex_input::BuffersDefinition,
            viewport::{
                ViewportState,
                Viewport
            }

        }

    },
    memory::{
        allocator::{
            StandardMemoryAllocator,
            GenericMemoryAllocator,
            FreeListAllocator
        }
    }





};
use error_stack::{Result, ResultExt};
use winit::window::Window;

use super::primitives::Vertex;
use super::surface::CSurface;
use crate::core::error_handler::{EVlkApi,ERendering};

//
//
pub(crate) struct VlkBase {

    instance:           Arc<Instance>,
    debug_callback:     DebugUtilsMessenger,
    pdevice:            Arc<PhysicalDevice>,
    qfamily_index:      u32,
    device:             Arc<Device>,
    queues:             Vec<Arc<Queue>>,
    images:             Vec<Arc<SwapchainImage>>,
    swapchain:          Arc<Swapchain>,
    pipeline:           Arc<GraphicsPipeline>,
    viewport:           Viewport,
    framebuffers:        Vec<Arc<ImageView<SwapchainImage>>>,
    cmd_buffer_alloc:   GenericMemoryAllocator<Arc<FreeListAllocator>>

}
//
impl VlkBase {

    pub(crate) fn init(
        app_name:           &str,
        app_version:        (u32,u32,u32),
        surf:               &CSurface,
        inst:               Arc<Instance> ) -> Result<Self,ERendering> {


        let dcallback = debug::init_debug_utils(inst.clone())
            .map_err(|e| e.change_context(ERendering::VLK_BASE))?;

        let (pdev,qfamilyindex) = Self::choose_pdevice(inst.clone(),surf.get_surface())
            .change_context(ERendering::VLK_BASE)?;

        let (device,queues) = Self::create_device_and_queues(pdev.clone(), qfamilyindex)
            .map_err(|e| e.change_context(ERendering::VLK_BASE))?;

        let (sc,imgs) = Self::create_swapchain_and_image(device.clone(),surf.get_surface())
            .map_err(|e| e.change_context(ERendering::VLK_BASE))?;

        // TODO: check for other shader types
        let svert = Self::load_vertex_shader(device.clone()).map_err(|e|
           e.change_context(ERendering::VLK_BASE)
        )?;

        let sfrag = Self::load_fragment_shader(device.clone()).map_err(|e|
            e.change_context(ERendering::VLK_BASE)
        )?;

        let pipeline = Self::create_graphic_pipeline(
            sc.clone(),
            sfrag.clone(),
            svert.clone(),
            device.clone()
        ).map_err(|e| e.change_context(ERendering::VLK_BASE))?;

        let mut viewport = Viewport {

            origin: [0.0,0.0],
            dimensions: [0.0,0.0],
            depth_range: 0.0..1.0

        };

        let framebuffers = window_size_dependent_setup(&imgs,&mut viewport);

        let cmd_buffer_alloc = StandardMemoryAllocator::new(
            device.clone(),
            Default::default()
        ).map_err(|e|
            EVlkApi::MEMORY
                .attach_printable_default(e)
                .change_context(ERendering::VLK_BASE)
        )?;

        Ok(
            Self {
                instance:       inst,
                debug_callback: dcallback,
                pdevice:        pdev,
                qfamily_index:  qfamilyindex,
                                queues,
                                device,
                swapchain:      sc,
                images:         imgs,
                                pipeline,
                                viewport,
                                framebuffers,
                                cmd_buffer_alloc

            }
        )

    }
    //
    pub(crate) fn get_image_queue(&self) -> Result<&Arc<Queue>,ERendering> {

        match self.queues.iter().next() {

            Some(q) => Ok(q),
            None => return Err(
                EVlkApi::QUEUE
                    .as_report()
                    .attach_printable("no queue could be found")
                    .change_context(ERendering::VLK_BASE)
            )
        }

    }
    //
    pub(crate) fn get_device(&self) -> Arc<Device> { self.device.clone() }
    //
    //
    //---------------------------------------------------------------------------------------------
    // Initialisation function
    //

    fn choose_pdevice(
        inst:   Arc<Instance>,
        surf:   &Arc<Surface>) -> Result<(Arc<PhysicalDevice>,u32), EVlkApi> {

        let dev_ext = DeviceExtensions {
            // TODO: add other features needed with maybe conditions
            khr_swapchain: true,
            ..DeviceExtensions::empty()

        };


        let filtering = inst
            .enumerate_physical_devices()
            .unwrap()
            .filter(|p| {

                p.api_version() >= Version::V1_3 || p.supported_extensions().khr_dynamic_rendering

            })
            .filter(|p| {

                p.supported_extensions().contains(&dev_ext)

            })
            .filter_map(|p| {

                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i,q)|{

                        q.queue_flags.intersects(&QueueFlags {
                            graphics: true,
                            ..Default::default()

                        }) && p.surface_support(i as u32, surf)
                            .unwrap_or(false)
                    })
                    .map(|i| (p,i as u32))
            });


        let (pdevice,qfamilyindex) = match filtering
            .min_by_key(|(p,_)| {

                match p.properties().device_type {

                    PhysicalDeviceType::DiscreteGpu => 0,
                    PhysicalDeviceType::IntegratedGpu => 1,
                    PhysicalDeviceType::VirtualGpu => 2,
                    PhysicalDeviceType::Cpu => 3,
                    PhysicalDeviceType::Other => 4,
                    _ => 5,

                }


            }) {

            Some(val) => val,
            None => return Err(EVlkApi::PHYSICAL_DEVICE
                .as_report()
                .attach_printable("No suitable physical device found")
            )


        };

        Ok((pdevice,qfamilyindex))

    }
    //
    fn create_device_and_queues(
        pdevice:        Arc<PhysicalDevice>,
        qfamily_index:  u32 ) -> Result<(Arc<Device>,Vec<Arc<Queue>>),EVlkApi> {

        let dev_ext = DeviceExtensions {
            // TODO: add other features needed with maybe conditions
            khr_swapchain: true,
            khr_dynamic_rendering:
            if pdevice.api_version() < Version::V1_3 {
                true
            } else {
                false
            },
            ..DeviceExtensions::default()
        };

        // return also an iterator of created queue
        let (dev,mut queue) = Device::new(

            pdevice,
            DeviceCreateInfo {

                enabled_extensions: dev_ext,
                enabled_features:  Features {
                    // TODO: add other features needed with maybe conditions
                    dynamic_rendering: true,
                    ..Features::default()

                },
                // TODO: check for other needed and possible queue
                queue_create_infos: vec![
                    // image Queue
                    QueueCreateInfo {
                        queue_family_index: qfamily_index,
                        ..Default::default()
                    }

                ],

                ..Default::default()

            }


        ).map_err(|e| EVlkApi::DEVICE
            .attach_printable_default(e)
        )?;


        Ok((dev,queue.collect()))
    }
    //
    fn create_swapchain_and_image(
        dev:    Arc<Device>,
        surf:   &Arc<Surface>) -> Result<(Arc<Swapchain>,Vec<Arc<SwapchainImage>>),EVlkApi> {

        let surface_capabilities = dev
            .physical_device()
            // TODO: check for important surface capabilities
            .surface_capabilities(surf,Default::default())
            .map_err(|e| EVlkApi::SWAPCHAIN
                .attach_printable_default(e)
            )?;


        // TODO: make able to choose the appropriate image format
        let img_format = Some(dev
            .physical_device()
            .surface_formats(surf,Default::default())
            .map_err(|e| EVlkApi::IMAGE
                .attach_printable_default(e)

            )?[0].0
        );

        let win_surface = match surf.object() {

            Some(obj) => {

                match obj.downcast_ref::<Window>() {

                    Some(w) => w,
                    None => return Err(EVlkApi::SURFACE
                        .as_report()
                        .attach_printable("cant downcast the surface obj \
                        parameters to winit::window::Window")

                    )

                }

            },
            None => return Err(EVlkApi::SURFACE
                .as_report()
                .attach_printable("cannot access object parameter of the surface")
            )


        };

        let (mut sc,imgs) = Swapchain::new(
            dev.clone(),
            surf.clone(),
            SwapchainCreateInfo {

                min_image_count:    surface_capabilities.min_image_count,
                image_format:       img_format,
                image_extent:       win_surface.inner_size().into(),
                // TODO: check for important image usage properties
                image_usage:        ImageUsage {
                    color_attachment: true,
                    ..Default::default()

                },
                composite_alpha: match surface_capabilities
                    .supported_composite_alpha.iter().next() {

                    Some(a) => a,
                    None => return Err(EVlkApi::SWAPCHAIN
                        .as_report()
                        .attach_printable("The surface has no composite alpha available")
                        )

                },
                ..Default::default()


            }

        ).map_err(|e| EVlkApi::SWAPCHAIN.attach_printable_default(e))?;

        Ok((sc,imgs))


    }

    fn load_vertex_shader(dev:Arc<Device>) -> Result<Arc<ShaderModule>,EVlkApi> {

        let s = VertexShader::load(dev).map_err(|e|
            EVlkApi::SHADER.attach_printable_default(e)
        )?;

        Ok(s)

    }

    fn load_fragment_shader(dev:Arc<Device>) -> Result<Arc<ShaderModule>,EVlkApi> {

        let s = VertexShader::load(dev).map_err(|e|
            EVlkApi::SHADER.attach_printable_default(e)

        )?;

        Ok(s)

    }

    fn create_graphic_pipeline(
        swapchain:Arc<Swapchain>,
        fragment_shader: Arc<ShaderModule>,
        vertex_shader:   Arc<ShaderModule>,
        dev: Arc<Device>
    ) -> Result<Arc<GraphicsPipeline>, EVlkApi> {

        vulkano::impl_vertex!(Vertex,position);

        // TODO: check for important pipeline rendering properties
        let graphic_pipeline_info = PipelineRenderingCreateInfo {

            color_attachment_formats: vec![Some(swapchain.image_format())],
            ..Default::default()


        };

        let pipeline = GraphicsPipeline::start()
            .render_pass(graphic_pipeline_info)
            .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
            .input_assembly_state(InputAssemblyState::new())
            .vertex_shader(match vertex_shader.entry_point("main") {

                Some(entry) => entry,
                None => return Err(
                    EVlkApi::SHADER
                        .as_report()
                        .attach_printable("Vertex shader have no entry function name \
                        'main'")

                )


            },())
            .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
            .fragment_shader(match fragment_shader.entry_point("main") {

                Some(entry) => entry,
                None => return Err(
                    EVlkApi::SHADER
                        .as_report()
                        .attach_printable("Fragment shader have no entry function name \
                                          'main'")
                    )
                }, ()
            )
            .build(dev)
            .map_err(|e| EVlkApi::GRAPHIC_PIPELINE.attach_printable_default(e))?;

        Ok(pipeline)

    }


}
//
//
mod VertexShader {
    vulkano_shaders::shader! {

        ty: "vertex",
        path: "data/test/base_sh.vert"

    }

}
//
//
mod FragmentShader {

    vulkano_shaders::shader! {

        ty: "fragment",
        path: "data/test/base_sh.frag"

    }

}
//
//
/// This method is called once during initialization, then again whenever the window is resized
fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage>],
    viewport: &mut Viewport,
) -> Vec<Arc<ImageView<SwapchainImage>>> {
    let dimensions = images[0].dimensions().width_height();
    viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];

    images
        .iter()
        .map(|image|  ImageView::new_default(image.clone()).unwrap())
        .collect::<Vec<_>>()

}



