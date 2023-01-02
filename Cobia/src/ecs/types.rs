#![allow(dead_code)]

// TODO: add comment

use std::ffi::CString;
use std::fs::{File, self};
use std::path::Path;
use std::io::BufReader;

use crate::ECobia;
use crate::renderer::primitives::*;
use crate::renderer::opengl::buffer::{TBO,VAO,VBO,EBO};
use crate::renderer::opengl::shader::{Source};
use crate::CWARN;

use super::EComponent;


// ------------------------------------------------------------------------------------------------
// General
//
// 
/// Common trait for all components of the engine
pub trait Component {
    //
    /// get the id of the component 
    fn get_id(&self) -> u32;
    //
    /// return the type of the component
    fn get_type(&self) -> CType;
    //
    /// set an id to the component
    fn set_id(&mut self, id: u32);
    //
    /// check if the component have an id other than 0 which means it is not initialized
    fn is_initialized(&self) -> bool;
    //
    //
}
//
//
/// All the possible types of components
pub enum CType {

    FILE,
    IMAGE,
    TEXTURE,
    MESH,
    BUFFER,
    SHADER,
    SOURCE,

}
//
//
// ------------------------------------------------------------------------------------------------
// File Type
//
// TODO: add possibility for mutable types 
//
//
/// lowest representation of a system file in the engine
pub(crate) struct CFile {

    id:         u32,    
    path:       String,
    ext:        String,
    content:    Vec<u8>
    
}
//
impl CFile {
    //
    //
    /// create a new Cfile struct 
    pub fn load(id: u32, path: &str) -> Result<Self,EComponent> {

        let ext = Self::get_file_extension(path)?;
   
        let content = match fs::read(path) {

            Ok(c) => c,
            Err(e) => {
                
                let cause = EComponent::FILE_CONTENT(e.to_string()).to_string();
                
                return Err(EComponent::LOADING_FILE { file: path.to_string(), cause: cause })
            
            }


        };

        Ok(CFile {
            id:         id,
            path:       path.to_string(),
            ext:        ext.to_string(),
            content:    content 
            }
        )
    }
    //
    /// get the file extension that is associated with this file
    pub fn get_extension(&self) -> &str { &self.ext }
    //
    /// get the file name that is associated with this file
    pub fn get_path(&self) -> &str { &self.path }
    //
    /// get an access to the file contents 
    pub fn get_access(&self) -> Result<File,EComponent> {
    
        match File::open(&self.path) {

            Ok(f) => Ok(f),
            Err(e) => return Err(EComponent::FILE_ACCESS(self.path.to_string(),e.to_string()))

        }
    
    }
    //
    /// Get the file's extension
    ///
    /// # Arguments
    ///
    /// * 'path' - File path to the file we want the extension
    ///  
    fn get_file_extension(path: &str) -> Result<&str, EComponent> {

        let p = Path::new(path);
    
        match p.extension() {
            Some(ext) => match ext.to_str() {
                Some(ext) => return Ok(ext),
    
                None => {
                    let e = ECobia::CONVERSION {
                        from: "OsStr".to_string(),
                        to: "str".to_string(),
                        how: format!("file extension of {}", path),
                    };

    
                    return Err(EComponent::LOADING_FILE{ 
                        file: path.to_string(),
                        cause: e.to_string()
                    });
                }
            },
    
            None => {
                
            
                return Err(EComponent::LOADING_FILE { 
                        file: path.to_string(),
                        cause: "Don't have extension".to_string()
                    }
                )
            
            },
        }
        
    }
    //
    //
    pub fn get_file_content(&self) -> &[u8] { &self.content }
    //
    //
}
//
impl Component for CFile {

    fn get_id(&self) -> u32 { self.id }
    fn get_type(&self) -> CType { CType::FILE }
    fn set_id(&mut self, id: u32) { self.id = id }
    fn is_initialized(&self) -> bool { self.id != 0 }


}
//
//
//
// ------------------------------------------------------------------------------------------------
// Image Types
//
//
/// Base of all image components
pub trait ImageTrait<T> {
    // The generic represent the bitrate
    // 
    /// initialize an image component
    fn new(id:u32,src:u32,w:u16,h:u16,data:Vec<T>) -> Self;
    //
    /// returns the widht of the image
    fn get_width(&self) -> u16;
    //
    /// return the height of the image
    fn get_height(&self) -> u16;
    //
    /// returns a flattened array of pixels
    fn get_data(&self) -> &Vec<T>;
    //
    /// return the number of value that represents one pixel in the image
    fn get_pixel_lenght(&self) -> u8;
    //
    /// return the file path of the image
    fn get_file_src(&self) -> u32;
    //
    //
}
//
//
#[derive(Debug)]
pub(crate) enum CImage {

    RGB8(Rgb8Image),
    RGB16(Rgb16Image),
    RGB32(Rgb32Image),
    RGBA8(Rgba8Image),
    RGBA16(Rgba16Image),
    RGBA32(Rgba32Image),

}
//
//
pub(crate) fn load_image(cfile:&CFile)-> Result<CImage,EComponent>
{

    match cfile.get_extension() {

        "png" => load_png(&cfile),

        "jpg" | "jpeg" => load_jpeg(&cfile),

        _ => Err(EComponent::LOAD_IMAGE(
            cfile.get_path().to_string(),
            format!("{} is not a valid extension for image",cfile.get_extension())
            )
        )
    }
    

}
//
//
fn load_png(cfile:&CFile) -> Result<CImage,EComponent> {



    let decoder = png::Decoder::new(cfile.get_access()?);
    let mut reader = match decoder.read_info() {

        Ok(r) => r,
        Err(e) => return Err(EComponent::LOAD_IMAGE(
            cfile.get_path().to_string(),
            e.to_string()
            )
        )

    };

    // check if its an APNG file
    if reader.info().animation_control.is_some() {

        CWARN!("doesn't support apng frame\nOnly the first frame will be taken");
    
    }

    let mut buf = vec![0; reader.output_buffer_size()];

    match reader.next_frame(&mut buf) {

        Ok(_) => {},
        Err(e) => return Err(EComponent::LOAD_IMAGE(
            cfile.get_path().to_string(),
            e.to_string()
            )
        ),

    }

    return match reader.info().color_type {

        png::ColorType::Rgb =>  {

            Ok(CImage::RGB8(
                Rgb8Image::new(
                    0,
                    cfile.get_id(),
                    reader.info().width as u16, 
                    reader.info().height as u16, 
                    buf)
                )
            )
            
        },

        png::ColorType::Rgba => {

            
            Ok(CImage::RGBA8(
                    Rgba8Image::new(
                        0,
                        cfile.get_id(),
                        reader.info().width as u16, 
                        reader.info().height as u16, 
                        buf
                    )
                )
            )
            
        },

        _ => Err(EComponent::LOAD_IMAGE(
            cfile.get_path().to_string(),
            "unsuported color type".to_string()
            )
        )
    }
    

}
//
//
fn load_jpeg(file: &CFile) -> Result<CImage,EComponent> {

    let mut decoder = jpeg_decoder::Decoder::new(BufReader::new(file.get_access()?));

    let data = match decoder.decode() {


        Ok(d) => d,
        Err(e) => return Err(EComponent::LOAD_IMAGE(
            file.get_path().to_string(),
            format!("Unable to access jpeg data by the decoder because of {}",e.to_string())
            )
        )

    };

    match decoder.info() {


        Some(info) => {

            
            match info.pixel_format {

                jpeg_decoder::PixelFormat::RGB24 => {

                    Ok(CImage::RGB8( 
                        Rgb8Image::new(
                            0,
                            file.get_id(),
                            info.width,
                            info.height,
                            data
                            )
                        )
                    )

                },

                _ => return Err(
                    EComponent::LOAD_IMAGE(
                        file.get_path().into(),
                        "pixel format not supported".into()
                    )
                )
            }

        },

        None => return Err(
            EComponent::LOAD_IMAGE(
                file.get_path().into(),
                "unable to access metadata of a jpeg/jpg file. Nothing was found".to_string()
            )
        )

    }

}
//
//
// ------------------------------------------------------------------------------------------------
// RGB8 image
//
//
#[derive(Debug,Clone)]
/// Lowest representation of an rgb8 image in this engine
pub(crate) struct Rgb8Image {

    id:         u32,
    src:        u32, // id of cfile struct 
    width:      u16,
    height:     u16,
    data:       Vec<u8>, 
    
}
//
impl Component for Rgb8Image { 

    fn get_id(&self) -> u32 { self.id }
    fn get_type(&self) -> CType { CType::IMAGE } 
    fn set_id(&mut self, id: u32) { self.id = id }
    fn is_initialized(&self) -> bool { self.id != 0 }

}
//
impl ImageTrait<u8> for Rgb8Image {

    fn new(id:u32,src:u32,w:u16,h:u16,data:Vec<u8>) -> Self {
        
        Rgb8Image {
            id:         id,
            src:        src,
            width:      w,
            height:     h,
            data:       data,
        }
    }

    fn get_height(&self) -> u16 { self.width }

    fn get_width(&self) -> u16 { self.height }

    fn get_data(&self) -> &Vec<u8> { &self.data }

    fn get_pixel_lenght(&self) -> u8 { 3_u8 }

    fn get_file_src(&self) -> u32 { self.src }

}
//
//
// ------------------------------------------------------------------------------------------------
// RGB16 image
//
//
#[derive(Debug,Clone)]
/// Lowest representation of an rgb16 image in this engine
pub(crate) struct Rgb16Image {

    id:         u32,
    src:        u32, // id of cfile struct
    width:      u16,
    height:     u16,
    data:       Vec<u16>, 
    
}
//
impl Component for Rgb16Image { 

    fn get_id(&self) -> u32 { self.id }
    fn get_type(&self) -> CType { CType::IMAGE }
    fn set_id(&mut self, id: u32) { self.id = id }
    fn is_initialized(&self) -> bool { self.id != 0 }
 
}
//
impl ImageTrait<u16> for Rgb16Image {

    fn new(id:u32,src:u32,w:u16,h:u16,data:Vec<u16>) -> Self {
        
        Rgb16Image { 
            
            id:     id,
            src:    src, // id of cfile struct
            width:  w,
            height: h,
            data:   data 
        }

    }

    fn get_height(&self) -> u16 { self.width }

    fn get_width(&self) -> u16 { self.height }

    fn get_data(&self) -> &Vec<u16> { &self.data }

    fn get_pixel_lenght(&self) -> u8 { 3_u8 }

    fn get_file_src(&self) -> u32 { self.src }

}
//
//
// ------------------------------------------------------------------------------------------------
// RGB32 image
//
//
#[derive(Debug,Clone)]
/// Lowest representation of an rgb32 image in this engine
pub(crate) struct Rgb32Image {

    id:         u32,
    src:        u32,
    width:      u16,
    height:     u16,
    data:       Vec<u32>, 
    
}
//
impl Component for Rgb32Image { 

    fn get_id(&self) -> u32 { self.id }
    fn get_type(&self) -> CType { CType::IMAGE }
    fn set_id(&mut self, id: u32) { self.id = id }
    fn is_initialized(&self) -> bool { self.id != 0 }

}
//
impl ImageTrait<u32> for Rgb32Image {


    fn new(id:u32,src:u32,w:u16,h:u16,data:Vec<u32>) -> Self {
        
        Rgb32Image { 
            id:     id,
            src:    src,
            width:  w,
            height: h,
            data:   data 
        }
    }

    fn get_height(&self) -> u16 { self.width }

    fn get_width(&self) -> u16 { self.height }

    fn get_data(&self) -> &Vec<u32> { &self.data }

    fn get_pixel_lenght(&self) -> u8 { 3_u8 }

    fn get_file_src(&self) -> u32 { self.src }

}
//
//
// ------------------------------------------------------------------------------------------------
// RGBA8 image
//
//
#[derive(Debug,Clone)]
/// Lowest representation of an rgba8 image in this engine
pub(crate) struct Rgba8Image {

    id:     u32,
    src:    u32, // id of cfile struct
    width:  u16,
    height: u16,
    data:   Vec<u8>, 

}
//
impl Component for Rgba8Image {
    
    fn get_id(&self) -> u32 { self.id }
    fn get_type(&self) -> CType { CType::IMAGE }
    fn set_id(&mut self, id: u32) { self.id = id }
    fn is_initialized(&self) -> bool { self.id != 0 }

}
//
impl ImageTrait<u8> for Rgba8Image {

    fn new(id:u32,src:u32,w:u16,h:u16,data:Vec<u8>) -> Self {
        
        Rgba8Image { 
            id:     id,
            src:    src,
            width:  w,
            height: h,
            data:   data 
        }
    }

    fn get_height(&self) -> u16 { self.width }

    fn get_width(&self) -> u16 { self.height }

    fn get_data(&self) -> &Vec<u8> { &self.data }

    fn get_pixel_lenght(&self) -> u8 { 4 }

    fn get_file_src(&self) -> u32 { self.src }

}
//
//
// ------------------------------------------------------------------------------------------------
// RGBA16 image
//
//
#[derive(Debug)]
/// Lowest representation of an rgba16 image in this engine
pub(crate) struct Rgba16Image {

    id:     u32,
    src:    u32, // id of cfile struct
    width:  u16,
    height: u16,
    data:   Vec<u16>,

}
//
impl Component for Rgba16Image {
    
    fn get_id(&self) -> u32 { self.id }
    fn get_type(&self) -> CType { CType::IMAGE }
    fn set_id(&mut self, id: u32) { self.id = id }
    fn is_initialized(&self) -> bool { self.id != 0 }

}
//
impl ImageTrait<u16> for Rgba16Image {

    fn new(id:u32,src:u32,w:u16,h:u16,data:Vec<u16>) -> Self {
        
        Rgba16Image { 
            id:     id,
            src:    src,
            width:  w,
            height: h,
            data:   data 
        }
    }

    fn get_height(&self) -> u16 { self.width }

    fn get_width(&self) -> u16 { self.height }

    fn get_data(&self) -> &Vec<u16> { &self.data }

    fn get_pixel_lenght(&self) -> u8 { 4 }

    fn get_file_src(&self) -> u32 { self.src }

}
//
//
// ------------------------------------------------------------------------------------------------
// RGBA32 image
//
//
#[derive(Debug)]
/// Lowest representation of an rgba32 image in this engine
pub(crate) struct Rgba32Image {

    id:     u32,
    src:    u32, // id of cfile struct
    width:  u16,
    height: u16,
    data:   Vec<u32>, 

}
//
impl Component for Rgba32Image {
    
    fn get_id(&self) -> u32 { self.id }
    fn get_type(&self) -> CType { CType::IMAGE }
    fn set_id(&mut self, id: u32) { self.id = id }
    fn is_initialized(&self) -> bool { self.id != 0 }

}
//
impl ImageTrait<u32> for Rgba32Image {

    fn new(id:u32,src:u32,w:u16,h:u16,data:Vec<u32>) -> Self {
        
        Rgba32Image { 
            id:     id,
            src:    src,
            width:  w,
            height: h,
            data:   data 
        }
    }

    fn get_height(&self) -> u16 { self.width }

    fn get_width(&self) -> u16 { self.height }

    fn get_data(&self) -> &Vec<u32> { &self.data }

    fn get_pixel_lenght(&self) -> u8 { 4 }

    fn get_file_src(&self) -> u32 { self.src }
    
}
//
//
// ------------------------------------------------------------------------------------------------
// Texture 
//
// 
pub(crate) trait TextureTraits {
    //
    /// get the component id of the image texture 
    fn get_src(&self) -> u32;
    //
    /// return the texture target for opengl calls
    fn get_target(&self) -> u32;
    //
    /// return the level of detail that opengl should interpret
    fn get_details_lvl(&self) -> u8;
    //
    /// access the opengl texture object that is linked to this struct 
    fn access_gl_buffer(&self) -> &TBO;
    //
    //
}
//
//
pub(crate) struct GTexture2D {

    id:             u32,
    src:            u32, // id of an image component
    BUFFER:         TBO, 
    details_lvl:    u8,

}
//
impl Component for GTexture2D {

    fn get_id(&self) -> u32 { self.id }
    fn get_type(&self) -> CType { CType::TEXTURE }
    fn set_id(&mut self, id: u32) { self.id = id }
    fn is_initialized(&self) -> bool { self.id != 0 }

}
//
impl TextureTraits for GTexture2D {

    fn get_details_lvl(&self) -> u8 { self.details_lvl }

    fn get_src(&self) -> u32 { self.src }

    fn get_target(&self) -> u32 { gl::TEXTURE_2D }

    fn access_gl_buffer(&self) -> &TBO { &self.BUFFER }

}
//
//



//
//
// ------------------------------------------------------------------------------------------------ 
// Graphics primitives buffer
//
//
/// Float Vertices
pub(crate) struct FVerticesBuffer{

    id:     u32,
    data:   Vec<FVertex>

}
//
impl Component for FVerticesBuffer {

    fn get_id(&self) -> u32 { self.id }
    fn get_type(&self) -> CType { CType::BUFFER }
    fn set_id(&mut self, id: u32) { self.id = id }
    fn is_initialized(&self) -> bool { self.id != 0 }

}
//
//

/// Double Vertices
pub(crate) struct DVerticesBuffer {

    id:     u32,
    data:   Vec<DVertex>

}
//
impl Component for DVerticesBuffer {

    fn get_id(&self) -> u32 { self.id }
    fn get_type(&self) -> CType { CType::BUFFER }
    fn set_id(&mut self, id: u32) { self.id = id }
    fn is_initialized(&self) -> bool { self.id != 0 }

}
//
//
pub(crate) struct IndicesBuffer{

    id:     u32,
    data:   Vec<Indice>
}
//
impl Component for IndicesBuffer {

    fn get_id(&self) -> u32 { self.id }
    fn get_type(&self) -> CType { CType::BUFFER }
    fn set_id(&mut self, id: u32) { self.id = id }
    fn is_initialized(&self) -> bool { self.id != 0 }

}
//
//
// ------------------------------------------------------------------------------------------------
// Mesh
//
//
pub trait MeshTrait {
    //
    /// return the id of the indices buffer
    fn get_indices(&self) -> u32;
    //
    /// return the id of the vertices buffer
    fn get_vertices(&self) -> u32;
    //
    //
}
//
//
pub(crate) struct GMesh {

    id:         u32,
    vertices:   u32,
    indices:    u32,
    vao:        VAO,
    vbo:        VBO,
    ebo:        EBO,


}
//
impl Component for GMesh { 

    fn get_id(&self) -> u32 { self.id } 
    fn get_type(&self) -> CType { CType::BUFFER }
    fn set_id(&mut self, id: u32) { self.id = id }
    fn is_initialized(&self) -> bool { self.id != 0 }
   
}
//
impl MeshTrait for GMesh {

    fn get_indices(&self) -> u32 { self.vertices }
    fn get_vertices(&self) -> u32 { self.vertices }
    

}
//
//
// ------------------------------------------------------------------------------------------------
// Shader
//
//
pub(crate) struct GShader {

    id:         u32,
    src:        u32, // id of a file component
    gsource:    u32  // opengl shader object

}
//
impl Component for GShader {

    fn get_id(&self) -> u32 { self.id }
    fn get_type(&self) -> CType { CType::SOURCE }
    fn set_id(&mut self, id: u32) { self.id = id }
    fn is_initialized(&self) -> bool { self.id != 0 }
    
}
//
impl GShader {

    pub fn new(file:&CFile) -> Result<Self,EComponent> {


        let content = Self::load_as_cstring(file.get_file_content())?;

        let stype = match file.get_extension() {

            "frag" => gl::FRAGMENT_SHADER,
            "vert" => gl::VERTEX_SHADER,

            _ => return Err(EComponent::SHADER_TYPE(file.get_extension().to_string()))


        };

        let gsource = Source::new(&content,stype)?;
        
        


    }
    

    fn load_as_cstring(file_content:&[u8]) -> Result<CString,EComponent> {

        match CString::new(file_content) {

            Ok(c) => Ok(c),
            Err(_) => Err(EComponent::from(
                ECobia::CONVERSION { 
                    from: "&[u8]".to_string(),
                    to: "CString".to_string(),
                    how: format!("get file content for gshader with id {}",self.id)
                    }
                )
            )

        }

    }
    
}
//
//
pub(crate) struct GShaderProgram {

    id:         u32,
    gsource:    Vec<u32>,
    gprogram:   []   

}
//
impl Component for GShaderProgram {

    fn get_id(&self) -> u32 { self.id }
    fn get_type(&self) -> CType { CType::SHADER }
    fn set_id(&mut self, id: u32) { self.id = id }
    fn is_initialized(&self) -> bool { self.id != 0 }
    
}
//
//
// ------------------------------------------------------------------------------------------------


