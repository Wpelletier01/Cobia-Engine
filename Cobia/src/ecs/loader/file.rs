
// TODO: add comment

use std::path::Path;
use std::fs::File;

use thiserror::Error;


use super::{CFile,ELoader};
use crate::ECobia;

//
//
// ------------------------------------------------------------------------------------------------
// Test
//
//
#[cfg(test)]
mod ftest {

    use super::*;
    
    use std::env;

    fn get_relative_path(path:&str) -> String {

        format!(
            "{}/{}",
            env::current_dir().unwrap().to_str().unwrap(),
            path
        )

    }

    #[test]
    fn load_dir() {

        let s = get_relative_path("data/test");
        
        let f = load_file(&s);
        
        assert!(f.is_err()) 
        
    }

    #[test]
    fn load_file_with_no_ext() {

        let s = get_relative_path("data/test/test_no_ext");

        let f =  load_file(&s);

        assert!(f.is_err())


    }

    #[test]
    fn correct_extension() {

        let s = get_relative_path("data/test/test1.png");

        let f = load_file(&s).unwrap();

        assert_eq!(f.get_extension(),"png");

    }


}
//
//
// ------------------------------------------------------------------------------------------------
// Error
//
//
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
// ------------------------------------------------------------------------------------------------
// Function
//
//
pub(crate) fn load_file(path:&str) -> Result<CFile,ELoader> {

    let ext = get_file_extension(path)?;

    Ok(
        CFile::new(
            0,
            path.to_string(),
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
pub fn get_file_extension(path: &str) -> Result<&str,ELoader> {
    
     
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

                    return Err(ELoader::from(e));

                }
            }
        },

        None => return Err(ELoader::from(EFile::FILE_EXT(path.to_string())))
    
    }
    //
}
//
//