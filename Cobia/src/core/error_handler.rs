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
    Conversion,
    MutexAccess,
    CStringParsing,

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
            
            Self::Conversion =>     write!(f, "Conversion Error"),
            Self::MutexAccess =>   write!(f, "MutexAccess Error"),
            Self::CStringParsing =>   write!(f,"C_STRING_PARSING Error")
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

    Logging,
    File,

}
//
impl fmt::Display for ECore { 

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {


            Self::Logging => write!(f, "Logging Module Error:"),
            Self::File => write!(f, "Load File Module Error:"),
           

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
    Extension,
    Content,
    Path
    
}
//
impl EFile {

    pub fn as_report(&self) -> Report<Self> { Report::new(*self)}

}
//
impl fmt::Display for EFile {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {

            Self::Extension =>  write!(f, "Extension Error"),
            Self::Content =>    write!(f, "Content Error"),
            Self::Path =>       write!(f, "Path error")

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
    VulkanBase,
    Surface,
    System,
    GpuRessources
}
//
impl ERendering {
    
    pub fn as_report(&self) -> Report<Self> { Report::new(*self)}


}
//
impl fmt::Display for ERendering { 


    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {

            Self::VulkanBase => write!(f, "Vulkan Rendering System"),
            Self::Surface =>  write!(f, "Window surface"),
            Self::System =>   write!(f, "Rendering System"),
            Self::GpuRessources => write!(f,"Free gpu resources")
            
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
    Library,
    Instance,
    Debug,
    PhysicalDevice,
    Device,
    Surface,
    Swapchain,
    SwapchainChange,
    GraphicPipeline,
    RenderPass,
    FrameBuffer,
    Queue,
    Image,
    Shader,
    Memory
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

            Self::Library => write!(f, "Api Entry Point Error:"),
            Self::Instance => write!(f, "Instance Error:"),
            Self::Debug => write!(f, "Debug utils Error:"),
            Self::PhysicalDevice => write!(f, "Physical Devices Error:"),
            Self::Device => write!(f, "Devices Error:"),
            Self::Surface => write!(f, "Vulkan Surface Error:"),
            Self::Swapchain => write!(f, "Swapchain Error:"),
            Self::GraphicPipeline => write!(f, "Graphics Pipeline Error:"),
            Self::Shader => write!(f, "Shader Module Error:"),
            Self::RenderPass => write!(f, "Renderpass Error:"),
            Self::FrameBuffer => write!(f, "FrameBuffer Error:"),
            Self::Queue => write!(f, "Queue Error:"),
            Self::Memory => write!(f, "Memory Error:"),
            Self::Image => write!(f, "Image Error:"),
            Self::SwapchainChange => write!(f,"Swapchain change Error:")
            
        }
    
    }
    
}
//
impl Context for EVlkApi {}
//
//
// ------------------------------------------------------------------------------------------------
