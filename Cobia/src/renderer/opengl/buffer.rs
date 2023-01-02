#![allow(dead_code)]

use std::ffi::c_void;
use std::mem;


use super::{EOpenGL,api};

type GLsizeiptr = isize;
//
//
// ------------------------------------------------------------------------------------------------
// Buffer Trait
// 
//
pub(crate) trait Binding {
    //
    /// set the buffer as the current one in use
    fn bind(&self) -> Result<(),EOpenGL>;
    //
    /// unset the buffer as the one in current use
    fn unbind(&self) -> Result<(),EOpenGL>;
    //
}
//
//
// ------------------------------------------------------------------------------------------------
// VERTEX ARRAY BUFFER
//
//
/// OpenGL vertex buffer object
pub(crate) struct VBO { id: u32 }
//
impl VBO {
    //
    /// Generates a new vertex buffer object 
    pub(crate) fn new() -> Result<Self,EOpenGL> {

        let mut id = 0;

        api::gen_buffer(1, &mut id)?;

        Ok(VBO { id: id })

    }
    //
    /// Pass vertex data to this vbo buffer 
    /// 
    /// # Arguments
    /// 
    /// * 'data'  - reference of a vector containing vertices
    /// * 'usage' - how this buffer will be used (eg. gl::STATIC_DRAW or gl::DYNAMIC_DRAW)
    /// 
    pub(crate) fn set_data(&self, data: &Vec<f32>,usage:gl::types::GLenum) -> Result<(),EOpenGL> {
        
        api::buffer_data(
            gl::ARRAY_BUFFER,
            (data.len()*mem::size_of::<f32>()) as gl::types::GLsizeiptr ,
            data.as_ptr() as *const c_void,
            usage
        )?;


        Ok(())

    }
    //
    //
}
//
impl Binding for VBO {

    fn bind(&self) -> Result<(),EOpenGL> {
        
        api::bind_vertex_array(self.id)?;

        Ok(())

    }

    fn unbind(&self) -> Result<(),EOpenGL> {
        
        api::bind_vertex_array(0)?;

        Ok(())

    }

}
//
//
// ------------------------------------------------------------------------------------------------
// VERTEX ATTRIBUTE BUFFER
//
//
/// OpenGL Vertex Array Buffer
pub(crate) struct VAO { id: u32 }
//
impl VAO {
    //
    //
    /// Generate an Vertex Array buffer
    pub(crate) fn new() -> Result<Self,EOpenGL> {

        let mut id:u32 = 0;

        api::gen_vertex_array(1, &mut id)?;


        Ok( VAO { id: id })


    }
    //
    //
    /// Set the vertex attribute pointer that will detail to OpenGL how to interpret each
    /// stripe of the array buffer
    /// 
    /// * 'index'           - number of vertex attributes that have been set
    /// * 'offset'          - where the attribute starts on each stripe
    /// * 'length'          - number of entries in the array that the attribute have 
    /// * 'stride_length'   - the number of entries in the array that represent one vertex
    /// 
    pub(crate) fn set_vertex_attribute_pointer(
        &mut self,
        index:          u32,
        offset:         usize,
        lenght:         i32,
        stride_length:  i32, ) -> Result<(),EOpenGL> {

        
        
        api::vertex_attrib_pointer(
            index,
            lenght,
            gl::FLOAT, 
            false, 
            stride_length*mem::size_of::<f32>() as i32, 
            (offset*mem::size_of::<f32>()) as *const c_void
        )?;

        api::enable_vertex_attrib_array(index)?;

        Ok(())
    }
    //
    //
}
//
impl Binding for VAO {

    fn bind(&self) -> Result<(),EOpenGL> {
        
        api::bind_vertex_array(self.id)?;

        Ok(())

    }

    fn unbind(&self) -> Result<(),EOpenGL> {
        
        api::bind_vertex_array(0)?;

        Ok(())

    }

}
//
//
// ------------------------------------------------------------------------------------------------
// ELEMENT ARRAY BUFFER
// 
//
/// OpenGL Element Array Object
pub(crate) struct EBO { id: u32 }
//
impl EBO {
    //
    // Generate a new element array buffer
    pub fn new() -> Result<EBO,EOpenGL> {

        let mut id:u32 = 0;

        api::gen_buffer(1,&mut id )?;


        Ok(EBO { id: id })

    }
    //
    /// allocate memory and store indices for the buffers
    ///
    /// # Arguments
    /// 
    /// * 'indices' - a reference to a vector that stores the indices
    /// * 'usage' - how this buffer will be used (eg. gl::STATIC_DRAW or gl::DYNAMIC_DRAW)
    /// 
    pub fn set_data(&self,indices:&Vec<u32>,usage:gl::types::GLenum) -> Result<(),EOpenGL> {

        api::buffer_data(
            gl::ELEMENT_ARRAY_BUFFER, (indices.len()*mem::size_of::<i32>()) as GLsizeiptr,
            indices.as_ptr() as *const c_void,
            usage
        )?;

        Ok(())
    }
    //
}
//
impl Binding for EBO {
    //
    fn bind(&self) -> Result<(),EOpenGL> {
        
        api::bind_buffer(gl::ELEMENT_ARRAY_BUFFER, self.id)?;

        Ok(())

    }
    //
    //
    fn unbind(&self) -> Result<(),EOpenGL> {
        
        api::bind_buffer(gl::ELEMENT_ARRAY_BUFFER,0)?;

        Ok(())

    }
    //
}
//
//
// ------------------------------------------------------------------------------------------------
// TEXTURE BUFFER
//
//
/// OpenGL Texture Object 
pub(crate) struct TBO { id: u32, target: u32 }
//
impl TBO {
    //
    //
    /// generate a texture buffer object
    /// 
    /// # Arguments
    /// 
    /// * 'target' - the type of texture to generate
    /// 
    fn new(target:u32) -> Result<Self,EOpenGL> {

         
        if ![
            
            gl::TEXTURE_2D,
            gl::PROXY_TEXTURE_2D,
            gl::TEXTURE_1D_ARRAY,
            gl::PROXY_TEXTURE_1D_ARRAY,
            gl::TEXTURE_RECTANGLE,
            gl::PROXY_TEXTURE_RECTANGLE,
            gl::TEXTURE_CUBE_MAP_POSITIVE_X,
            gl::TEXTURE_CUBE_MAP_NEGATIVE_X, 
            gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
            gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
            gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
            gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
            gl::PROXY_TEXTURE_CUBE_MAP

        ].contains(&target) {

            return Err(
                EOpenGL::BUFFER(
                    "texture".to_string(),
                    "invalid target for image source 2d".to_string()
                )
            );

   
        }


        let mut id:u32 = 0;

        api::gen_textures(1, &mut id)?;

        Ok(TBO { id: id, target: target })

    }
    //
    //
    /// assign an image to the texture buffer object
    /// 
    /// # Arguments
    /// 
    /// * 'details_lvl' - Specifies the level-of-detail number
    /// * 'internal_fmt'  - Specifies the number of color components in the texture
    /// * 'width'         - width of the image
    /// * 'height'        - height of the image
    /// * 'gl_format'     - Specifies the format of the pixel data
    /// * 'gl_type'       - Specifies the data type of the pixel data
    /// 
    fn set_2d_image_source<T>(
        &self,
        details_lvl:  i32,
        internal_fmt:   i32,
        width:          i32,
        height:         i32,
        gl_format:      u32,
        gl_type:        u32,   
        data:           *const T ) -> Result<(),EOpenGL> 
    {
        
        //
        // cant have details_lvl other than zero with the this type of target
        if [gl::TEXTURE_RECTANGLE,gl::PROXY_TEXTURE_RECTANGLE].contains(&self.target) 
            && details_lvl != 0 {

            return Err(EOpenGL::BUFFER(
                "texture".to_string(),
                "details_lvl must be 0 if target is ether gl::TEXTURE_RECTANGLE or
                gl::PROXY_TEXTURE_RECTANGLE".to_string()

            ));                

        }
        //
        // 
        // validate the internal format parameters
        if ![
            gl::DEPTH_COMPONENT,
            gl::DEPTH_STENCIL,
            gl::RED,
            gl::RG,
            gl::RGB,
            gl::RGBA 
        ].contains(&(internal_fmt as u32)) {

            return Err(
                EOpenGL::BUFFER(
                    "texture".to_string(),
                    "invalid intenal_fmt parameter".to_string()

                ));

        }
        //
        //
        // validate gl_format parameter
        if ![ 
            gl::RED, gl::RG,  gl::RGB, gl::BGR, gl::RGBA, gl::BGRA, 
            gl::RED_INTEGER,  gl::RG_INTEGER, gl::RGB_INTEGER, gl::BGR_INTEGER,
            gl::RGBA_INTEGER, gl::BGRA_INTEGER, gl::STENCIL_INDEX, gl::DEPTH_COMPONENT, 
            gl::DEPTH_STENCIL 
        
        ].contains(&gl_format) {

            return Err(EOpenGL::BUFFER(
                "texture".to_string(), 
                "invalid gl_format parameter".to_string()
            ));

        }
        //
        //
        // validate gl_type parameter
        if ![ 

            gl::UNSIGNED_BYTE, 
            gl::BYTE, 
            gl::UNSIGNED_SHORT, 
            gl::SHORT,
            gl::UNSIGNED_INT, 
            gl::INT,
            gl::HALF_FLOAT, 
            gl::FLOAT, 
            gl::UNSIGNED_BYTE_3_3_2, 
            gl::UNSIGNED_BYTE_2_3_3_REV, 
            gl::UNSIGNED_SHORT_5_6_5, 
            gl::UNSIGNED_SHORT_5_6_5_REV, 
            gl::UNSIGNED_SHORT_4_4_4_4,
            gl::UNSIGNED_SHORT_4_4_4_4_REV, 
            gl::UNSIGNED_SHORT_5_5_5_1, 
            gl::UNSIGNED_SHORT_1_5_5_5_REV, 
            gl::UNSIGNED_INT_8_8_8_8, 
            gl::UNSIGNED_INT_8_8_8_8_REV, 
            gl::UNSIGNED_INT_10_10_10_2, 
            gl::UNSIGNED_INT_2_10_10_10_REV

        ].contains(&gl_type) {

            return Err(EOpenGL::BUFFER(
                "texture".to_string(),
                "invalid gl_type parameter".to_string()
                )
            );


        }
        //
        // then call the OpenGL function 
        //
        api::tex_image_2d(
            self.target,
            details_lvl,
            internal_fmt,
            width,
            height,
            0,
            gl_format, 
            gl_type,
            data as *const _
        )?;

        Ok(())
        //
    }
    //
}
//
impl Binding for TBO {
    
    fn bind(&self) -> Result<(),EOpenGL> {

        // TODO: make it possible to other dimension type texture to work
        api::bind_textures(gl::TEXTURE_2D, self.id)?;

        Ok(())

    }

    fn unbind(&self) -> Result<(),EOpenGL> {
        
        // TODO: make it possible to other dimension type texture to work
        api::bind_textures(gl::TEXTURE_2D, 0)?;

        Ok(())

    }

}
//
//
// ------------------------------------------------------------------------------------------------
//