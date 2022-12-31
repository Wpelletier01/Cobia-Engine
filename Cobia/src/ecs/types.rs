use std::fs::File;





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
    /// return the type of the component
    fn get_type(&self) -> CType;
    //
}
//
//
/// All the possible types of components
pub enum CType {

    FILE,
    IMAGE,
    TEXTURE

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

    fn get_type(&self) -> CType { CType::FILE }

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
pub(crate) enum Images {

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
/// Lowest representation of an rgb16 image in this engine
pub(crate) struct Rgb16Image {

    id:         u32,
    src:        u32,
    width:      u16,
    height:     u16,
    data:       Vec<u16>, 
    
}
//
impl Component for Rgb16Image { 

    fn get_id(&self) -> u32 { self.id }

    fn get_type(&self) -> CType { CType::IMAGE } 

}
//
impl ImageTrait<u16> for Rgb16Image {

    fn new(id:u32,src:u32,w:u16,h:u16,data:Vec<u16>) -> Self {
        
        Rgb16Image { 
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

    fn get_pixel_lenght(&self) -> u8 { 3_u8 }

    fn get_file_src(&self) -> u32 { self.src }

}
//
//
// ------------------------------------------------------------------------------------------------
// RGB32 image
//
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
/// Lowest representation of an rgba8 image in this engine
pub(crate) struct Rgba8Image {

    id:     u32,
    src:    u32,
    width:  u16,
    height: u16,
    data:   Vec<u8>, 

}
//
impl Component for Rgba8Image {
    
    fn get_id(&self) -> u32 { self.id }

    fn get_type(&self) -> CType { CType::IMAGE } 

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
/// Lowest representation of an rgba16 image in this engine
pub(crate) struct Rgba16Image {

    id:     u32,
    src:    u32,
    width:  u16,
    height: u16,
    data:   Vec<u16>,

}
//
impl Component for Rgba16Image {
    
    fn get_id(&self) -> u32 { self.id }

    fn get_type(&self) -> CType { CType::IMAGE } 

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
/// Lowest representation of an rgba32 image in this engine
pub(crate) struct Rgba32Image {

    id:     u32,
    src:    u32,
    width:  u16,
    height: u16,
    data:   Vec<u32>, 

}
//
impl Component for Rgba32Image {
    
    fn get_id(&self) -> u32 { self.id }

    fn get_type(&self) -> CType { CType::IMAGE } 

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

//
//
