#![allow(dead_code)]
#![allow(unused_imports)]

use std::fmt;



use error_stack::{Context, IntoReport, Report, ResultExt};

#[cfg(test)]
mod tests {
    use super::*;
    


    #[derive(Debug)]
    pub(crate) enum GenErr1  {

        TEST1 


    }

    impl fmt::Display for GenErr1 {

        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

            f.write_str("GenError1 occurred: ")

        }

    }

    impl Context for GenErr1 {}


    #[derive(Debug)]
    pub(crate) enum GenErr2 {

        TEST2 

    }

    impl fmt::Display for GenErr2 {

        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

            f.write_str("GenError2 occurred: ")

        }

    }

    impl Context for GenErr2 {}




    pub(crate) fn stack1() -> error_stack::Result<(),GenErr1> {

        let num:u32= 11;

        stack2(num).change_context(GenErr1::TEST1)
            .attach_printable("stack1 func cant do his job")?;


        Ok(())

    }


    pub(crate) fn stack2(num:u32) -> error_stack::Result<(),GenErr2> {


        if num > 10 {

            return Err(Report::new(GenErr2::TEST2)
                .attach_printable("error from stack2"));

        }

        Ok(())

    }


    #[test]
    fn error() {

        stack1().unwrap();


    }

}
//
//
// ------------------------------------------------------------------------------------------------ 
// General Error types (any error type that is created outside of the control of the Engine ) 
//
//
#[allow(non_camel_case_types)]
#[derive(Debug,Clone, Copy)]
pub(crate) enum EGeneral  {
    
    CONVERSION,
    MUTEX_ACCESS 

}
//
impl EGeneral {

    pub(crate) fn as_report(&self) -> Report<Self> { Report::new(*self) }

}
//
impl fmt::Display for EGeneral {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            
            Self::CONVERSION =>     write!(f, "EGeneral::Conversion Error:"),
            Self::MUTEX_ACCESS =>   write!(f, "EGeneral::MutexAccess Error:"),

        }


    }


}
//
impl Context for EGeneral {}
//
//
// ------------------------------------------------------------------------------------------------
// Core module (High Level error that occurs in the core module)
// 
//
#[derive(Debug)]
pub enum ECore {

    LOGGING 

}
//
impl fmt::Display for ECore { 

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {


            Self::LOGGING => write!(f, "Logging Module Error:"),


        }


    }

}
//
impl Context for ECore {}
//
//
// ------------------------------------------------------------------------------------------------

