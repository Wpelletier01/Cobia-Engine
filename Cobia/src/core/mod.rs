

pub mod logs;
pub(crate) mod application;



#[cfg(test)]
mod test {

    use super::*;

    #[cfg(test)]
    mod test_log {

        
        
        use super::logs;
               
        use logs::*;
        /* 
        #[test]
        fn t_fmt_log() {

            let s = "[00:00:00:00] [FATAL]: this is a test";

            assert_eq!(
                s,
                fmt_log(Level::FATAL, "this is a test".to_string())
            );

        }

        #[test]
        #[should_panic]
        fn not_initialized() {

            push_log("not initialize before calling",Level::FATAL);

        }

        #[test]
        fn test_output_look() {

            init();

            logs::push_log("ahh check the look of this INFO  thang", Level::INFO);
            logs::push_log("ahh check the look of this DEBUG thang", Level::DEBUG);
            logs::push_log("ahh check the look of this ERROR thang", Level::ERROR);
            logs::push_log("ahh check the look of this WARN  thang", Level::WARN);
            logs::push_log("ahh check the look of this TRACE thang", Level::TRACE);
            logs::push_log("ahh check the look of this FATAL thang", Level::FATAL);

            assert!(true);

        }

        */

    }




}