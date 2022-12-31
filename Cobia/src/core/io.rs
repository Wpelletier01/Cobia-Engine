
use crate::{CERROR,CWARN};
use thiserror::Error;
use std::path::Path;

#[allow(non_camel_case_types)]
#[derive(Error, Debug)]
pub enum EIo {

    #[error("The path {0} is invalid. Reason: {1}")]
    BAD_PATH(String,String),
    #[error("Cant access the file {0} extension. Possible cause: 
        - there is no file name
        - there is no embedded '.'
    ")]
    FILE_EXT(String),
  
}
//
//
/// make sure that the path exist and its a file
///
/// # Arguments
/// 
/// * 'fp' - a file to a path
/// 
fn load_file(fp:&str) -> Result<&Path,EIo> {

    let p = Path::new(fp);

    match p.try_exists() {

        Ok(rep) => {
            
            if !rep {

                if p.is_symlink()  {
                    return Err(EIo::BAD_PATH(fp.to_string(),"Broken symbolic link".to_string()));
                }

                return Err(EIo::BAD_PATH(fp.to_string(),"file doesn't exist".to_string()));
                
            }
        },

        Err(e) => return Err(EIo::BAD_PATH(fp.to_string(),e.to_string())),



    }

    Ok(p)

}
//
//
/// Get the file's extension
/// 
/// # Arguments
/// 
/// * 'path' - File path to the file we want the extension
///  
pub fn get_file_extension(path: &str) -> Result<&str,EIo> {
    //
     
    let p = Path::new(path);

    match p.extension() {

        Some(ext) => {

            match ext.to_str() {

                Some(ext) => return Ok(ext),

                None => return Err(
                    EIo::BAD_PATH(path.to_string(),"cant parse OsStr to str".to_string())
                )
            }
        },

        None => return Err(EIo::FILE_EXT(path.to_string()))
    
    }
    //
}