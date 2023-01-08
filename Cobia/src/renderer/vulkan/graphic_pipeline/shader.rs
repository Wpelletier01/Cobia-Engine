

use ash::vk;

use super::{EVlkGraphicPipeline,Result};

use std::{ptr, ffi::CString};



pub(crate) struct ShaderStages { shaders: Vec<vk::PipelineShaderStageCreateInfo> }

impl ShaderStages {

    pub(crate) fn init() -> Self { ShaderStages { shaders: Vec::new() } }

    pub(crate) fn add_shader(
        &mut self,
        dev:    &ash::Device,
        shader: vk::ShaderModule,
        type_:  vk::ShaderStageFlags) {
        
        let main_entry = CString::new("main").unwrap();

        let pipe_shader_info = vk::PipelineShaderStageCreateInfo {

            s_type:                 vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            p_next:                 ptr::null(),
            flags:                  vk::PipelineShaderStageCreateFlags::empty(),
            module:                 shader,
            p_name:                 main_entry.as_ptr(),
            p_specialization_info:  ptr::null(),
            stage:                  type_

        };

        self.shaders.push(pipe_shader_info); 

        unsafe {
            dev.destroy_shader_module(shader, None);
        }

        
    }

    



}
//
//
// ------------------------------------------------------------------------------------------------
// Shader Module 
//
//
pub(crate) fn create_shader_module(
    data:       Vec<u8>,
    main_entry: &str, 
    dev:        &ash::Device ) -> Result<vk::ShaderModule,EVlkGraphicPipeline> {

    let create_info = vk::ShaderModuleCreateInfo{

        s_type:     vk::StructureType::SHADER_MODULE_CREATE_INFO,
        p_next:     ptr::null(),
        flags:      vk::ShaderModuleCreateFlags::empty(),
        code_size:  data.len(),
        p_code:     data.as_ptr() as *const u32 

    };


    let shader_module = unsafe {
        
        match dev.create_shader_module(&create_info, None) {

            Ok(v) => v,
            Err(e) => return Err(EVlkGraphicPipeline::SHADER_MODULE
                    .as_report()
                    .attach_printable(format!(
                        "Can't create shader module info because of {}",
                        e.to_string()
                        )
                    )
                )

        }
            
    };

    Ok(shader_module)

}
//
//
// ------------------------------------------------------------------------------------------------
// Shader info
//
pub(crate) struct ShaderInfo {

    type_: vk::ShaderStageFlags,
    data:  Vec<u8>

}