#![allow(dead_code)]
#![allow(unused_imports)]

use std::fmt;
use std::fmt::write;

use error_stack::{Context, IntoReport, Report, ResultExt};

#[cfg(test)]
mod tests {
    use super::*;
    


    #[derive(Debug)]
    pub(crate) enum GenErr1  {

        TEST1 


    }

    impl fmt::Display for GenErr1 {

        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

            f.write_str("GenError1 occurred: ")

        }

    }

    impl Context for GenErr1 {}


    #[derive(Debug)]
    pub(crate) enum GenErr2 {

        TEST2 

    }

    impl fmt::Display for GenErr2 {

        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

            f.write_str("GenError2 occurred: ")

        }

    }

    impl Context for GenErr2 {}




    pub(crate) fn stack1() -> error_stack::Result<(),GenErr1> {

        let num:u32= 11;

        stack2(num).change_context(GenErr1::TEST1)
            .attach_printable("stack1 func cant do his job")?;


        Ok(())

    }


    pub(crate) fn stack2(num:u32) -> error_stack::Result<(),GenErr2> {


        if num > 10 {

            return Err(Report::new(GenErr2::TEST2)
                .attach_printable("error from stack2"));

        }

        Ok(())

    }


    #[test]
    fn error() {

        stack1().unwrap();


    }

}
//
//
// ------------------------------------------------------------------------------------------------ 
// General Error types (any error type that is created outside of the control of the Engine ) 
//
//
#[allow(non_camel_case_types)]
#[derive(Debug,Clone, Copy)]
pub(crate) enum EGeneral  {
    
    CONVERSION,
    MUTEX_ACCESS,
    C_STRING_PARSING, 

}
//
impl EGeneral {

    pub(crate) fn as_report(&self) -> Report<Self> { Report::new(*self) }

    pub fn attach_printable_default<T:std::error::Error>(&self,error:T) -> Report<Self> {

        self.as_report().attach_printable(error.to_string())

    }

}
//
impl fmt::Display for EGeneral {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            
            Self::CONVERSION =>     write!(f,       "Conversion Error"),
            Self::MUTEX_ACCESS =>   write!(f,       "MutexAccess Error"),
            Self::C_STRING_PARSING =>   write!(f,   "C_STRING_PARSING Error")
        }


    }


}
//
impl Context for EGeneral {}
//
//
// ------------------------------------------------------------------------------------------------
// Core module (High Level error that occurs in the core module)
// 
//
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum ECore {

    LOGGING,
    FILE,


}
//
impl fmt::Display for ECore { 

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {


            Self::LOGGING => write!(f, "Logging Module Error:"),
            Self::FILE => write!(f, "Load File Module Error:"),
           

        }


    }

}
//
impl Context for ECore {}
//
//
// 
#[derive(Debug,Clone, Copy)]
pub enum EFile {

    EXTENSION,
    CONTENT,
    PATH
    
}
//
impl EFile {

    pub fn as_report(&self) -> Report<Self> { Report::new(*self)}

}
//
impl fmt::Display for EFile {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {

            Self::EXTENSION =>  write!(f, "Extension Error"),
            Self::CONTENT =>    write!(f, "Content Error"),
            Self::PATH =>       write!(f, "Path error")

        }
    }

}
//
impl Context for EFile {}
//
//
// ------------------------------------------------------------------------------------------------
// Rendering module (High Level error that occurs in the rendering module)
// 
//
#[derive(Debug,Clone, Copy)]
pub enum ERendering {

    VLK_BASE,
    SURFACE,
    SYSTEM
}
//
impl ERendering {
    
    pub fn as_report(&self) -> Report<Self> { Report::new(*self)}


}
//
impl fmt::Display for ERendering { 


    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {

            Self::VLK_BASE => write!(f,"Vulkan Rendering System"),
            Self::SURFACE =>  write!(f,"Window surface"),
            Self::SYSTEM =>   write!(f,"Rendering System")
            
        }

    }

}
//
impl Context for ERendering {}
//
//
// ------------------------------------------------------------------------------------------------
// Error that occurs when dealing with the vulkan Api
// 
//
#[allow(non_camel_case_types)]
#[derive(Debug,Clone, Copy)]
pub enum EVlkApi {

    LIBRARY,
    INSTANCE,
    DEBUG,
    PHYSICAL_DEVICE,
    DEVICE,
    SURFACE,
    SWAPCHAIN,
    GRAPHIC_PIPELINE,
    RENDERPASS,
    FRAMEBUFFER,
    QUEUE,
    IMAGE,
    SHADER,
    MEMORY


}
//
impl EVlkApi {

    pub fn as_report(&self) -> Report<Self> { Report::new(*self)}

    pub fn attach_printable_default<T:std::error::Error>(&self,error:T) -> Report<Self> {

        self.as_report().attach_printable(error.to_string())

    }

}
//
impl fmt::Display for EVlkApi {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        match self {

            Self::LIBRARY => write!(f,"Api Entry Point Error:"),
            Self::INSTANCE => write!(f, "Instance Error:"),
            Self::DEBUG => write!(f, "Debug utils Error:"),
            Self::PHYSICAL_DEVICE => write!(f, "Physical Devices Error:"),
            Self::DEVICE => write!(f, "Devices Error:"),
            Self::SURFACE => write!(f, "Vulkan Surface Error:"),
            Self::SWAPCHAIN => write!(f, "Swapchain Error:"),
            Self::GRAPHIC_PIPELINE => write!(f, "Graphics Pipeline Error:"),
            Self::SHADER => write!(f, "Shader Module Error:"),
            Self::RENDERPASS => write!(f, "Renderpass Error:"),
            Self::FRAMEBUFFER => write!(f,"FrameBuffer Error:"),
            Self::QUEUE => write!(f,"Queue Error:"),
            Self::MEMORY => write!(f,"Memory Error:"),
            Self::IMAGE => write!(f,"Image Error:")
            
        }
    
    }
    
}
//
impl Context for EVlkApi {}
//
//
// ------------------------------------------------------------------------------------------------
