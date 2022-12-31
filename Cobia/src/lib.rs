#[macro_use]
#[allow(dead_code)]

pub mod define;
pub mod ecs;
pub mod renderer;

pub(crate) mod core;








#[cfg(test)]
mod quick_test {

    use std::env;

    fn get_relative_path(p:&str) -> String {
        format!("{}/{}",env::current_dir().unwrap().to_str().unwrap(),p)
    }

    #[test]
    fn tmain() {

        use std::fs::File;

        let t = get_relative_path("/data/test/test_image.png");

        // The decoder is a build for reader and can be used to set various decoding options
        // via `Transformations`. The default output transformation is `Transformations::IDENTITY`.
        let decoder = png::Decoder::new(File::open(t).unwrap());
        let mut reader = decoder.read_info().unwrap();
        // Allocate the output buffer.
        let mut buf = vec![0; reader.output_buffer_size()];
        // Read the next frame. An APNG might contain multiple frames.
        let info = reader.next_frame(&mut buf).unwrap();
        // Grab the bytes of the image.
        let bytes = &buf[..info.buffer_size()];
        // Inspect more details of the last read frame.
        let in_animation = reader.info().frame_control.is_some();

        
     
        println!("{:?}",info);


        assert!(in_animation);

    }



}
