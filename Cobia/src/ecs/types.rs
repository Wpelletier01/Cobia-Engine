#![allow(dead_code)]


use std::fs::File;


use crate::renderer::primitives::*;

use crate::renderer::opengl::buffer::{TBO,VAO,VBO,EBO};


// ------------------------------------------------------------------------------------------------
// General
//
// 
/// Common trait for all components of the engine
pub trait Component {
    //
    /// get th id of the component 
    /// TODO: maybe it represents the index of the component in an array of components?
    fn get_id(&self) -> u32;
    //
  
}
//
//
/// All the possible types of components
pub enum CType {

    FILE    (CFile),
    IMAGE   (CImage),
    TEXTURE (CTexture),
    MESH    (CMesh),
    GBUFFER (CBuffer),
    SHADER  (CShader),
    SOURCE  (CSource),

}
//
//
// ------------------------------------------------------------------------------------------------
// File Type
//
// TODO: add possibility for mutable types 
//
/// lowest representation of a system file in the engine
pub(crate) struct CFile {

    id:         u32,    
    path:       String,
    access:     File,
    ext:        String
    
}
//
impl CFile {
    //
    //
    /// create a new Cfile struct 
    pub fn new(id: u32, path: String,access:File,ext:String) -> Self {

        CFile {
            id:         id,
            path:       path,
            access:     access,
            ext:        ext
        }
    }
    //
    /// get the file extension that is associated with this file
    pub fn get_extension(&self) -> &str { &self.ext }
    //
    /// get the file name that is associated with this file
    pub fn get_path(&self) -> &str { &self.path }
    //
    /// get an access to the file contents 
    pub fn get_access(&self) -> &File { &self.access }
    //
    //
}
//
impl Component for CFile {

    fn get_id(&self) -> u32 { self.id }

 

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
// ------------------------------------------------------------------------------------------------
// RGB8 image
//
//
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
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
pub(crate) trait GTexture {
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
pub enum CTexture {

    GL2D(GTexture2D)

}

//
//
pub(crate) struct GTexture2D {

    id:             u32,
    src:            u32, // id of an image component
    gbuffer:        TBO, 
    details_lvl:    u8,

}
//
impl Component for GTexture2D {

    fn get_id(&self) -> u32 { self.id }

}
//
impl GTexture for GTexture2D {

    fn get_details_lvl(&self) -> u8 { self.details_lvl }

    fn get_src(&self) -> u32 { self.src }

    fn get_target(&self) -> u32 { gl::TEXTURE_2D }

    fn access_gl_buffer(&self) -> &TBO { &self.gbuffer }

}
//
//



//
//
// ------------------------------------------------------------------------------------------------ 
// Graphics primitives buffer
//
//
pub enum CBuffer {

    FVERT(FVerticesBuffer),
    DVERT(DVerticesBuffer),
    INDICE(IndicesBuffer)

}

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
   
}
//
//
// ------------------------------------------------------------------------------------------------
// Mesh
//
//

pub enum CMesh {

    GMESH(GMesh)
}
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
pub enum CSource {

    GSOURCE(GSource)

}

//
//
pub(crate) struct GSource {

    id:     u32,
    
    src:    u32, // id of a file component
    gid:    u32  // source id

}
//
impl Component for GSource {

    fn get_id(&self) -> u32 { self.id }
    
}
//
impl GSource {

    


}
//
//
pub enum CShader {

    GSHADER(GShader)

}
//
//
pub(crate) struct GShader {

    id:     u32,
    source: Vec<u32>  

}
//


//
//


