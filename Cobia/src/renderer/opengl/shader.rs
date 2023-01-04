
use std::ffi::CString;

// TODO: add comment

use super::{api,EOpenGL};

use ShaderParser::ShaderFileInfo;


pub struct Source {

    id:         u32,
    compiled:   bool,

}
//
impl Source {
    //
    //
    pub fn new(content:&CString,type_:gl::types::GLenum) -> Result<Source,EOpenGL> {


        let id = api::create_shader(type_)?;

        api::shader_source(&id, &content)?;
                
        Ok(Source { id: id, compiled: false })

    }
    //
    //
    pub fn compile(&mut self) -> Result<(),EOpenGL> {

        api::compile_shader(&self.id)?;

        self.check_compile_status()?;

        self.compiled = true;

        Ok(())

    }
    //
    //
    fn check_compile_status(&self) -> Result<(),EOpenGL> {

        let status = api::get_shader_iv(
            &self.id, 
            gl::COMPILE_STATUS )?;


        if status == 0 {
            
            let error_log_size:i32 = api::get_shader_iv(&self.id,gl::INFO_LOG_LENGTH)?;

            println!("{}",error_log_size);

            let info_log:Vec<u8> = Vec::with_capacity(error_log_size as usize);

            let msg = api::get_shader_info_log(&self.id,info_log,error_log_size)?;
        
            return Err(EOpenGL::COMPILE_SHADER(msg));
        }

        Ok(())

    }
    //
    pub fn delete(&self) -> Result<(),EOpenGL> {

        api::delete_shader(&self.id)?;

        Ok(())

    }
    //
    pub fn is_compiled(&self) -> bool { self.compiled }
    //
    pub fn get_gid(&self) -> u32 { self.id}
    //
}
//
//
//
pub struct Program {

    id:     u32,

}
//
impl Program {
    //
    //
    pub fn new() -> Result<Self,EOpenGL> {
        
        let id =  api::create_program()?;
        Ok( Program { id: id } )

    }
    //
    //
    pub fn attach_shader(&self, shader: &Source) -> Result<(),EOpenGL> {

        api::attach_shader(&self.id, &shader.get_gid())?;

        Ok(())

    }
    //
    //
    pub fn link(&self) -> Result<(),EOpenGL> {

        api::link_program(&self.id)?;

        Ok(())

    }
    //
    //
    pub fn make_current(&self) -> Result<(),EOpenGL> {

        api::use_program(&self.id)?;

        Ok(())

    }   
    //
    //
    pub fn get_id(&self) -> u32 { self.id }
    //
    //
    pub fn set_int_var(&self,var_name:&str, value:i32) -> Result<(),EOpenGL> {


        let c_var = CString::new(var_name).unwrap();


        let location  = api::get_uniform_location(&self.id, c_var)?;

        api::uniform_int_1(location, value)?;
        
        Ok(())


    }
    //
    //
}