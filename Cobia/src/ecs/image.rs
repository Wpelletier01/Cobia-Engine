
use std::fs::File;

use super::{EComponent,get_next_id};
use crate::{CWARN};

use super::types::*;
//
//
// TODO: Add comment 
//
//
// ------------------------------------------------------------------------------------------------
// Test 
//
//

//
//
// ------------------------------------------------------------------------------------------------
// Function
//
//
pub(crate) fn load_image(cfile:&CFile)-> Result<Images,EComponent>
{

    match cfile.get_extension() {

        "png" => load_png(&cfile),

        _ => Err(EComponent::LOAD_IMAGE(
            cfile.get_path().to_string(),
            format!("{} is not a valid extension for image",cfile.get_extension())
            )
        )
    }
    

}
//
//
fn load_png(cfile:&CFile) -> Result<Images,EComponent> {



    let decoder = png::Decoder::new(cfile.get_access());
    let mut reader = match decoder.read_info() {

        Ok(r) => r,
        Err(e) => return Err(EComponent::LOAD_IMAGE(
            cfile.get_path().to_string(),
            e.to_string()
            )
        )

    };

    // check if its a apng file
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

            Ok(Images::RGB8(
                Rgb8Image::new(
                    get_next_id(),
                    cfile.get_id(),
                    reader.info().width as u16, 
                    reader.info().height as u16, 
                    buf)
                )
            )
            
        },

        png::ColorType::Rgba => {

            
            Ok(Images::RGBA8(
                    Rgba8Image::new(
                        get_next_id(),
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
fn load_jpeg(file: File) -> Result<(),EComponent> {




    Ok(())

}