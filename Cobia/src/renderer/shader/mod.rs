
use ShaderParser::EParser;
use thiserror::Error;

use crate::ECobia;



#[derive(Error, Debug)]
pub enum EShader {

    #[error("Error when parsing shader: {source} ")]
    PARSER {
        #[from]
        source: EParser

    },

    #[error("General Error caused by: {source} ")]
    GENERAL {

        #[from]
        source: ECobia,

    }


}
//
