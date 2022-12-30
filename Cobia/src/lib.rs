#[macro_use]
#[allow(dead_code)]

pub mod define;
pub mod ecs;
pub mod renderer;

pub(crate) mod core;








#[cfg(test)]
mod quick_test {

    use image::io::Reader as ImageReader;
    use std::env;

    fn get_relative_path(p:&str) -> String {
        format!("{}/{}",env::current_dir().unwrap().to_str().unwrap(),p)
    }

    #[test]
    fn tmain() {

        let p = get_relative_path("data/test/test_image.png");

        let img = ImageReader::open(p).unwrap().decode().unwrap();

        let mut bytes: Vec<u8> = Vec::new();
        println!("{:?}",img.color());
        //println!("{:?}",img.as_flat_samples_u8().unwrap())

        


    }



}
