#[macro_use]
#[allow(dead_code)]

pub mod define;
pub mod ecs;
pub mod renderer;

pub mod core;


use thiserror::Error;
//
//
// ------------------------------------------------------------------------------------------------ 
// Notes 
/*
    
    g prefix for variables or struct represented something that use the opengl renderer api
    E prefix for Error enum 

*/
//
// ------------------------------------------------------------------------------------------------
// TODO List 
/* 

    TODO: make error accepting more generic types
    TODO: add log entry throughout 

*/
//
// ------------------------------------------------------------------------------------------------
//
//
/// function for making some test more convenient
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

    use winit::event_loop::{ControlFlow, EventLoop};
    use winit::window::WindowBuilder;   

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


    #[test]
    fn opengl_playfield() {




    }



}

