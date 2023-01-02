#[macro_use]
#[allow(dead_code)]

pub mod define;
pub mod ecs;
pub mod renderer;

pub(crate) mod core;


use thiserror::Error;



// TODO: make error accepting more generic types


/// function for making testing more convenient
pub(crate) mod test_helper {

    use std::env;

    pub(crate) fn get_relative_path(p:&str) -> String {
        format!("{}/{}",env::current_dir().unwrap().to_str().unwrap(),p)
    }


}


#[cfg(test)]
mod quick_test {

    use std::fs::File;
    use std::io::BufReader;

    use super::test_helper;

    #[test]
    fn tmain() {

        let s = test_helper::get_relative_path("data/test/test2.jpg");

        let file = File::open(s).expect("failed to open file");
        let mut decoder = jpeg_decoder::Decoder::new(BufReader::new(file));
        let pixels = decoder.decode().expect("failed to decode image");
        let metadata = decoder.info().unwrap();

        println!("{:?}",metadata);

    }



}


/// All the general error tha can be thrown at not only one module
#[derive(Debug,Error)] 
pub enum ECobia {

    #[error("Can't convert {from} to {to} while accessing {access}")]
    ConversionError{ from: String, to: String, access: String },



}
