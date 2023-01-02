// TODO: Add comment 

use std::io::BufReader;

use super::{ELoader,get_next_id};
use crate::{CWARN};

use super::super::types::*;


//
//
// ------------------------------------------------------------------------------------------------
// Test 
//
//
#[cfg(test)]
mod itest {

    use super::*;
    use crate::test_helper;
    use super::super::file::load_file;

    use super::{ImageTrait,super::super::types::Component};

    #[test]
    fn unsuported_image_type() {

        let s = test_helper::get_relative_path("data/test/test3.webp");

        let f = load_file(&s).unwrap();

        let img = load_image(&f);


        assert!(img.is_err())


    }

    #[test]
    fn image_data_quick_test() {

        let s = test_helper::get_relative_path("data/test/test2.jpg");

        let f = load_file(&s).unwrap();

        let img = load_image(&f).unwrap();


        match img {

            CImage::RGB8(_img) => {

                println!("image data store in RGB8 struct");

                println!("id of cfile component: {}", _img.get_file_src());
                println!("height: {}", _img.get_height());
                println!("widht: {}", _img.get_width());
                println!("pixel lenght: {}", _img.get_pixel_lenght());

                println!("Component type: {:?}", _img.get_type());
                println!("id of the image component: {}", _img.get_id());

            },
            CImage::RGBA8(_img) => {

                println!("image data store in RGBA8 struct");

                println!("id of cfile component: {}", _img.get_file_src());
                println!("height: {}", _img.get_height());
                println!("widht: {}", _img.get_width());
                println!("pixel lenght: {}", _img.get_pixel_lenght());

                println!("Component type: {:?}", _img.get_type());
                println!("id of the image component: {}", _img.get_id());

            },

            CImage::RGB16(_img) => {

                println!("image data store in RGB16 struct");

                println!("id of cfile component: {}", _img.get_file_src());
                println!("height: {}", _img.get_height());
                println!("widht: {}", _img.get_width());
                println!("pixel lenght: {}", _img.get_pixel_lenght());

                println!("Component type: {:?}", _img.get_type());
                println!("id of the image component: {}", _img.get_id());

            },

            CImage::RGBA16(_img) => {

                println!("image data store in RGBA16 struct");

                println!("id of cfile component: {}", _img.get_file_src());
                println!("height: {}", _img.get_height());
                println!("widht: {}", _img.get_width());
                println!("pixel lenght: {}", _img.get_pixel_lenght());

                println!("Component type: {:?}", _img.get_type());
                println!("id of the image component: {}", _img.get_id());

            },

            CImage::RGB32(_img) => {

                println!("image data store in RGB32 struct");

                println!("id of cfile component: {}", _img.get_file_src());
                println!("height: {}", _img.get_height());
                println!("widht: {}", _img.get_width());
                println!("pixel lenght: {}", _img.get_pixel_lenght());

                println!("Component type: {:?}", _img.get_type());
                println!("id of the image component: {}", _img.get_id());

            },

            CImage::RGBA32(_img) => {

                println!("image data store in RGBA32 struct");
                

                println!("id of cfile component: {}", _img.get_file_src());
                println!("height: {}", _img.get_height());
                println!("widht: {}", _img.get_width());
                println!("pixel lenght: {}", _img.get_pixel_lenght());

                println!("Component type: {:?}", _img.get_type());
                println!("id of the image component: {}", _img.get_id());
                

            },


        }

    }



    #[test]
    fn load_a_correct_png() {

        let s = test_helper::get_relative_path("data/test/test1.png");

        let f = load_file(&s).unwrap();

        let img = load_image(&f).unwrap();

   

        match img {

            CImage::RGBA8(_) => assert!(true),

            _ => {

            
                assert!(false)

            },


        }


    }


}
//
//
// ------------------------------------------------------------------------------------------------
// Function
//
//
pub(crate) fn load_image(cfile:&CFile)-> Result<CImage,ELoader>
{

    match cfile.get_extension() {

        "png" => load_png(&cfile),

        "jpg" | "jpeg" => load_jpeg(&cfile),

        _ => Err(ELoader::LOAD_IMAGE(
            cfile.get_path().to_string(),
            format!("{} is not a valid extension for image",cfile.get_extension())
            )
        )
    }
    

}
//
//
fn load_png(cfile:&CFile) -> Result<CImage,ELoader> {



    let decoder = png::Decoder::new(cfile.get_access());
    let mut reader = match decoder.read_info() {

        Ok(r) => r,
        Err(e) => return Err(ELoader::LOAD_IMAGE(
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
        Err(e) => return Err(ELoader::LOAD_IMAGE(
            cfile.get_path().to_string(),
            e.to_string()
            )
        ),

    }

    return match reader.info().color_type {

        png::ColorType::Rgb =>  {

            Ok(CImage::RGB8(
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

            
            Ok(CImage::RGBA8(
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

        _ => Err(ELoader::LOAD_IMAGE(
            cfile.get_path().to_string(),
            "unsuported color type".to_string()
            )
        )
    }
    

}
//
//
fn load_jpeg(file: &CFile) -> Result<CImage,ELoader> {

    let mut decoder = jpeg_decoder::Decoder::new(BufReader::new(file.get_access()));

    let data = match decoder.decode() {


        Ok(d) => d,
        Err(e) => return Err(ELoader::LOAD_IMAGE(
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
                            get_next_id(),
                            file.get_id(),
                            info.width,
                            info.height,
                            data
                            )
                        )
                    )

                },

                _ => return Err(
                    ELoader::LOAD_IMAGE(
                        file.get_path().into(),
                        "pixel format not supported".into()
                    )
                )


            }


        },

        None => return Err(
            ELoader::LOAD_IMAGE(
                file.get_path().into(),
                "unable to access metadata of a jpeg/jpg file. Nothing was found".to_string()
            )
        )


    }



}