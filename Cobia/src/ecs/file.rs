

use std::convert::AsRef;
use std::path::Path;
use std::fs::File;

use thiserror::Error;


use super::types::CFile;
use super::EComponent;
use crate::ECobia;


#[allow(non_camel_case_types)]
#[derive(Error, Debug)]
pub enum EFile {

    #[error("File Error caused when acceding by: {source}")]
    GENERAL{

        #[from]
        source: ECobia,
        

    },
    
    #[error("Cant access the file {0} extension. Possible cause: 
        - there is no file name
        - there is no embedded '.'
        - other 
    ")]
    FILE_EXT(String),
    
    #[error("Cant open/load the file {file} because: {cause}")]
    FILE_LOAD{ file: String, cause: String }
  
}



//
//
pub(crate) fn load_file(path:&str) -> Result<CFile,EComponent> {

    let access = match File::open(path) {

        Ok(f) => f,

        Err(e) => return Err( EComponent::from(EFile::FILE_LOAD { 
                file: path.to_string(),
                cause: e.to_string() 
            }) 
        )
    };

    let ext = get_file_extension(path)?;



    Ok(
        CFile::new(
            super::get_next_id(),
            path.to_string(),
            access,
            ext.to_string()        
        )

    )

}
//
//
/// Get the file's extension
/// 
/// # Arguments
/// 
/// * 'path' - File path to the file we want the extension
///  
pub fn get_file_extension(path: &str) -> Result<&str,EComponent> {
    //
     
    let p = Path::new(path);

    match p.extension() {

        Some(ext) => {

            match ext.to_str() {

                Some(ext) => return Ok(ext),

                None => {

                    let e = EFile::from( 
                        ECobia::ConversionError { 
                            from: "OsStr".to_string(), 
                            to: "str".to_string(), 
                            access: format!("file extension of {}",path) 
                        }
                    );

                    return Err(EComponent::from(e));

                }
            }
        },

        None => return Err(EComponent::from(EFile::FILE_EXT(path.to_string())))
    
    }
    //
}
//
//