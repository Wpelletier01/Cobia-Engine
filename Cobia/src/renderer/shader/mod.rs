
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
//
pub(crate) fn get_file_info(content: &[u8]) -> Result<(),EShader> {

    let scontent = match String::from_utf8(content.to_vec()) {

        Ok(c) => c,
        Err(e) => return Err(EShader::from( ECobia::CONVERSION { 
            from:   "vec<u8>".into(),
            to:     "String".into(), 
            how:    "maybe a NullCharacter".into() }
            )
        )

    };

    

    Ok(())

}