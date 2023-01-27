
use std::path::Path; 
use std::fs;

use error_stack::{Result, ResultExt};
use super::error_handler::{EFile,EGeneral};
//
//
//
/// check if the path exist
///
/// # Parameters
///
/// * 'path' - the path to the file to be validated
///
fn is_a_correct_path(path: &str) -> Result<(),EFile> {
    //
    let p = Path::new(path);

    // check if the file exists and is allow
    match p.try_exists()  {

        Ok(rep) => {
            
            if !rep {

                if p.is_symlink()  {
                    return Err( 
                        EFile::PATH
                            .as_report()
                            .attach_printable(format!(
                                "The path {} is a broken symbolic link",
                                path
                                )
                            )
                        );
                }

                return Err( 
                    EFile::PATH
                        .as_report()
                        .attach_printable(format!(
                            "{} does not exist",
                            path
                            )
                        )
                    );
                

            }


        },

        Err(e) => return Err(
            EFile::PATH
               .as_report()
               .attach_printable(format!(
                    "{}",
                    e.to_string()
                    )
                )
            ),

    }

    Ok(())

}
//
//
/// return the file extension from an file path
///
/// # Parameters
///
/// * 'fp' - A file path that contain the extension to be extracted
///
pub(crate) fn get_file_extension(fp:&str) -> Result<&str,EFile> {
    //
    // Check first if the path passed is valid
    is_a_correct_path(fp)
        .change_context(EFile::EXTENSION)
        .attach_printable("Can't find file extension")?;

    let p = Path::new(fp);
    
    match p.extension() {
        //
        Some(_ext) => {
            //  
            // convert to &str
            match _ext.to_str() {
                Some(ex) => Ok(ex),
                None => return Err(
                    EGeneral::CONVERSION
                    .as_report()
                    .attach_printable("Couldn't convert OsStr to str")
                    .change_context(EFile::EXTENSION)
                    .attach("Can't find file extension")
                    ),

            }
        },
        //
        // weird rare case
        None => return Err(
            EFile::EXTENSION
               .as_report()
               .attach_printable(format!(
                    "
                    unable to retrieve file extension for {}. Possible causes:\n\t
                    - hadn't a file name\n\t
                    - don't have a dot\n\t
                    - have dot but nothing after
                    ",
                    fp
                    )
                )
            )
        //
    }
    //
}
//
//
/// get the content file and return it as bytes
///
/// # Parameters
///
/// * 'fp' - A file path to extract his content
///
pub(crate) fn get_file_content(fp:&str) -> Result<Vec<u8>,EFile> {
    //
    // check first that the file path passed is valid
    is_a_correct_path(fp)
        .change_context(EFile::CONTENT)
        .attach_printable("Can't get file content")?;

    match fs::read(fp) {

        Ok(ctn) => Ok(ctn),

        Err(e) => return Err(
            
            EFile::CONTENT
               .as_report()
               .attach_printable(format!(
                    "cant access file {} content because of {}",
                    fp,
                    e
                    )
                )

            )

    }

}
//
//