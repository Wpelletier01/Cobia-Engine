
use std::fs::File;

use super::{CType,Component,EComponent,get_next_id};
use crate::{core::io::get_file_extension, CERROR, CWARN};


//
//
/// Base of all image components
pub trait ImageTrait<T> {
    // The generic represent the bitrate
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
// Function
//
/// load a image file in is given image component 
/// 
/// * 'fp' file path to the image 
/// 
pub(crate) fn load_image<T>(fp:&str)-> Result<Images,EComponent>
where T: ImageTrait<T>
{

    match get_file_extension(fp) {

        Ok(ext) => {

            match ext {

                "png" => load_png(fp),
                
                _ => return Err(
                    EComponent::LOAD_IMAGE(
                        fp.to_string(),
                        "file extension not supported".to_string()
                    )
                )



            }


        },

        Err(e) => return Err(EComponent::LOAD_IMAGE(fp.to_string(),e.to_string()))

    }

    

}
//
//
fn load_png(fp:&str) -> Result<Images,EComponent> {

    let f = match File::open(fp) {

        Ok(f) => f,
        Err(e) => return Err(EComponent::LOAD_IMAGE(fp.to_string(),e.to_string()))

    };

    let decoder = png::Decoder::new(f);
    let mut reader = match decoder.read_info() {

        Ok(r) => r,
        Err(e) => return Err(EComponent::LOAD_IMAGE(fp.to_string(),e.to_string()))

    };

    // check if its a apng file
    if reader.info().animation_control.is_some() {

        CWARN!("doesn't support apng frame\nOnly the first frame will be taken");
    
    }

    let mut buf = vec![0; reader.output_buffer_size()];

    match reader.next_frame(&mut buf) {

        Ok(_) => {},
        Err(e) => return Err(EComponent::LOAD_IMAGE(fp.to_string(),e.to_string())),

    }


    return match reader.info().color_type {

        png::ColorType::Rgb =>  {

            Ok(Images::RGB8(
                Rgb8Image {
                    id: get_next_id(),
                    src: fp.to_string(),
                    width: reader.info().width as u16,
                    height: reader.info().height as u16,
                    data: buf 
                }
            
                )
            )
            
        },

        png::ColorType::Rgba => {

            Ok(Images::RGBA8(
                Rgba8Image {
                    id:     get_next_id(), 
                    src:    fp.to_string(),
                    width:  reader.info().width as u16,
                    height: reader.info().height as u16, 
                    data:   buf 
                }
                )
            )


        },

        _ => Err(EComponent::LOAD_IMAGE(fp.to_string(),"unsuported color type".to_string()))
    }
    

}
//
//
// ------------------------------------------------------------------------------------------------
// RGB8 image
//
/// Lowest representation of an rgb8 image in this engine
pub(crate) struct Rgb8Image {

    id:         u32,
    src:        String,
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

    fn get_height(&self) -> u16 { self.width }

    fn get_width(&self) -> u16 { self.height }

    fn get_data(&self) -> &Vec<u8> { &self.data }

    fn get_pixel_lenght(&self) -> u8 { 3_u8 }

}
//
//
// ------------------------------------------------------------------------------------------------
// RGB16 image
//
/// Lowest representation of an rgb16 image in this engine
pub(crate) struct Rgb16Image {

    id:         u32,
    src:        String,
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

    fn get_height(&self) -> u16 { self.width }

    fn get_width(&self) -> u16 { self.height }

    fn get_data(&self) -> &Vec<u16> { &self.data }

    fn get_pixel_lenght(&self) -> u8 { 3_u8 }

}
//
//
// ------------------------------------------------------------------------------------------------
// RGB32 image
//
/// Lowest representation of an rgb16 image in this engine
pub(crate) struct Rgb32Image {

    id:         u32,
    src:        String,
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

    fn get_height(&self) -> u16 { self.width }

    fn get_width(&self) -> u16 { self.height }

    fn get_data(&self) -> &Vec<u32> { &self.data }

    fn get_pixel_lenght(&self) -> u8 { 3_u8 }

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
    src:    String,
    width:  u16,
    height: u16,
    data:   Vec<u8>, // The generic represent the bitrate

}
//
impl Component for Rgba8Image {
    
    fn get_id(&self) -> u32 { self.id }

    fn get_type(&self) -> CType { CType::IMAGE } 

}
//
impl ImageTrait<u8> for Rgba8Image {

    fn get_height(&self) -> u16 { self.width }

    fn get_width(&self) -> u16 { self.height }

    fn get_data(&self) -> &Vec<u8> { &self.data }

    fn get_pixel_lenght(&self) -> u8 { 4 }

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
    src:    String,
    width:  u16,
    height: u16,
    data:   Vec<u16>, // The generic represent the bitrate

}
//
impl Component for Rgba16Image {
    
    fn get_id(&self) -> u32 { self.id }

    fn get_type(&self) -> CType { CType::IMAGE } 

}
//
impl ImageTrait<u16> for Rgba16Image {

    fn get_height(&self) -> u16 { self.width }

    fn get_width(&self) -> u16 { self.height }

    fn get_data(&self) -> &Vec<u16> { &self.data }

    fn get_pixel_lenght(&self) -> u8 { 4 }

}
//
//
// ------------------------------------------------------------------------------------------------
// RGBA32 image
//
//
/// Lowest representation of an rgba16 image in this engine
pub(crate) struct Rgba32Image {

    id:     u32,
    src:    String,
    width:  u16,
    height: u16,
    data:   Vec<u32>, // The generic represent the bitrate

}
//
impl Component for Rgba32Image {
    
    fn get_id(&self) -> u32 { self.id }

    fn get_type(&self) -> CType { CType::IMAGE } 

}
//
impl ImageTrait<u32> for Rgba32Image {

    fn get_height(&self) -> u16 { self.width }

    fn get_width(&self) -> u16 { self.height }

    fn get_data(&self) -> &Vec<u32> { &self.data }

    fn get_pixel_lenght(&self) -> u8 { 4 }

}
//
//
// ------------------------------------------------------------------------------------------------