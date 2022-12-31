#[macro_use]
#[allow(dead_code)]

pub mod define;
pub mod ecs;
pub mod renderer;

pub(crate) mod core;


use thiserror::Error;
use std::convert::AsRef;


// TODO: make error accepting more generic types


#[cfg(test)]
mod quick_test {

    use std::env;

    fn get_relative_path(p:&str) -> String {
        format!("{}/{}",env::current_dir().unwrap().to_str().unwrap(),p)
    }

    #[test]
    fn tmain() {



    }



}


/// All the general error tha can be thrown at not only one module
#[derive(Debug,Error)] 
pub enum ECobia {

    #[error("Can't convert {from} to {to} while accessing {access}")]
    ConversionError{ from: String, to: String, access: String },



}
