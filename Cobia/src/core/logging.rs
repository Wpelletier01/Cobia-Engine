
use crate::define::{CRELEASE,FMIN_AS_SECONDS,FHOUR_AS_SECONDS};
use super::apps_state::get_prog_elapsed_time;

use std::sync::atomic::{AtomicU8,Ordering};
use std::sync::Mutex;
use std::time::Duration;

use colored::Colorize;


// ------------------------------------------------------------------------------------------------
// Constant and Static  
// 
// They are by default all enabled
static      LOG_WARN_ENABLE:    AtomicU8    = AtomicU8::new(1);
static      LOG_INFO_ENABLE:    AtomicU8    = AtomicU8::new(1);
static      LOG_DEBUG_ENABLE:   AtomicU8    = AtomicU8::new(1);
static      LOG_TRACE_ENABLE:   AtomicU8    = AtomicU8::new(1);
//
const       LOG_BUFFER_SIZE:    usize       = 20000;
const       LOG_MAX_QUEUE_SIZE: usize       = 300;
const       MAX_LINE_LEN:       usize       = 90;
const       CFAILURE:           u8          = 0;
const       CSUCCESS:           u8          = 1;
//
const CLEVEL_STRING:[&str;6] = ["[FATAL]:","[ERROR]:","[WARN]: ","[INFO]: ","[DEBUG]:","[TRACE]"];
//
const TAB_MESSAGE: &str = "\n                       "; // 23 columns of whitespace
//
//
lazy_static::lazy_static! { 
    
    static ref LOG_QUEUE: Mutex<LogQueue> =   Mutex::new(LogQueue::new());
    static ref LOG_INIT:  Mutex<u8>       =   Mutex::new(0);       
    
}
//
// ------------------------------------------------------------------------------------------------
// Basic functions 
//
/// initialize stuff for enabling the logging subsystem. if not called,
/// nothing will work
pub(crate) fn init_logging(){
    //
    // assure that logging is enabled
    *LOG_INIT.lock().unwrap() = 1;
    //
    // check if in release mode and if so disable debug and trace logging
    if CRELEASE == 1{

        LOG_DEBUG_ENABLE.store(0,Ordering::Release);
        LOG_TRACE_ENABLE.store(0,Ordering::Release);

    }
    // add the initial log
    let start = Log::new(Level::INFO,"START".purple());

    push_log(start);
    //
}
//
//
/// Check if init_logging have been called
pub fn is_init() {

    if *LOG_INIT.lock().unwrap() == 0 { 
        panic!("You need to call init_logging() before doing anything else with the log module");
    }

}
//
//
/// Add a log entry to the end of the LOG_QUEUE  
/// 
/// # Parameters
/// 
/// * log - a log entry to be added to the queue
/// 
fn push_log(log: Log) {
    //
    // ensure that the subsystem is initialized
    is_init();
    // logging to the terminal
    // TODO: remove this when gui module or disable by default
    log.print();
    //
    //
    match LOG_QUEUE.lock() {

        Ok(q) => q.append(log),
        Err(err) => 
            // No choice for now because we don't have any other way to communicate failing logs 
            // push but log failure should not crash the whole application
            panic!("Failed to push log to the queue\nReason: {}",err.to_string())

    }
    //
}
//
//
// ------------------------------------------------------------------------------------------------
// Formatting functions
//
//
/// Parsing log entry to a string
/// 
/// # Parameters
/// 
/// * level - the type of log
/// * message - what the log says
/// 
fn fmt_log(level: Level, message: String) -> String{
    //
    // level represent the index in the CLEVEL_STRING
    // " {TIME} {TYPE} {MESSAGE}"
    let msg = format!(
        "{} {} {}",
        fmt_duration_log(get_prog_elapsed_time()),
        CLEVEL_STRING[level as usize],
        message
    );

    msg
    //
}
//
//
/// Format the duration entry to a string 
/// 
/// # Parameters
/// 
/// * dur - Duration since Engine initialized
/// 
pub fn fmt_duration_log(dur:Duration) -> String {

    let mut secs = dur.as_secs_f32();
    let mut min:u32 = 0;
    let mut hour:u32 = 0;

    if secs >= FHOUR_AS_SECONDS {

        hour = (secs / FHOUR_AS_SECONDS) as u32;
        secs -= FHOUR_AS_SECONDS*hour as f32;

    }

    if secs >= FMIN_AS_SECONDS {

        min = (secs / FMIN_AS_SECONDS) as u32;
        secs -= FMIN_AS_SECONDS*min as f32;

    }

    let millis_sec = ((secs - (secs as u32) as f32 ) * 100_f32) as u32;

    let sec = secs as u32;

    let f_hour =    format_single_digit_value(hour);
    let f_min =     format_single_digit_value(min);
    let f_secs =    format_single_digit_value(sec);
    let f_millis =  format_single_digit_value(millis_sec);

    format!("[{}:{}:{}:{}]",f_hour,f_min,f_secs,f_millis)

}
//
//
fn format_single_digit_value(value: u32) -> String {

    if value < 9{ return format!("0{}",value); }

    format!("{}",value)

}
//
// ------------------------------------------------------------------------------------------------
//
//
pub struct LogQueue(Vec<Log>);
//
impl LogQueue {

    fn new() -> Self {
        let q :Vec<Log> = Vec::with_capacity(LOG_MAX_QUEUE_SIZE);
        LogQueue(q)

    }

    fn append(&mut self,log:Log) {

        if self.0.len() + 1 == LOG_MAX_QUEUE_SIZE {
            //TODO: write the log queue to a file and clear it
            unimplemented!();

        }

        self.0.push(log);


    }
}
//
//
pub struct Log(String);
//
impl Log{
    //
    #[allow(unused_assignments)]
    pub fn new(level:Level, message:ColoredString) -> Self{

        let mut n_msg = String::new();
        
        if message.len() > MAX_LINE_LEN {
            
            let msg_color = message.fgcolor().unwrap(); 

            let nb_lines = message.len() as f32 / MAX_LINE_LEN as f32;
       
            let mut v: Vec<String> = Vec::new();

            for line in 0..nb_lines.ceil() as i32 {

                
                let tmp_str = message[line as usize*MAX_LINE_LEN..].to_string();
          
                if tmp_str.len() < MAX_LINE_LEN{
                    v.push(tmp_str[..tmp_str.len()].to_string());
                }else{
                    
                    v.push(tmp_str[..90].to_string());
                }
        
            }
            n_msg = v.join(TAB_MESSAGE).color(msg_color).to_string();
         

        }else{
        
            n_msg = message.to_string();

        }

    
        // Check if the len of the message is bigger than the max allowed 
        let mut msg =  fmt_log(level,n_msg);
       
        if msg.len() > LOG_BUFFER_SIZE - 1 {

            msg = msg[..LOG_BUFFER_SIZE].to_string();

        }

      
        Log(msg)

    }
    
    pub fn print(&self) {

        if LOG_DEBUG_ENABLE.load(Ordering::Relaxed) == 1{
            println!("{}",&self.0);
        }

    }

    pub fn get(&self) -> &String { &self.0 }

}
//
//
#[repr(usize)]
#[derive(PartialEq)]
pub enum Level{

    FATAL = 0,
    ERROR = 1,
    WARN  = 2,
    INFO  = 3,
    DEBUG = 4,
    TRACE = 5,

}
//
//
#[macro_export]
macro_rules! CFATAL {
    //
    //
    ($fmt_string:expr) => {

        use crate::Core::logger::{send_log,Level::FATAL};
        
        {

            let _mess:&str = $fmt_string; // make sure that a &str is passed

        }

        send_log($fmt_string,FATAL);


    };
    //
    //
    ($fmt_string:expr, $( $arg:expr ),*) => {
        use crate::Core::logger::{send_log,validate_msg,Level::FATAL};
      
        {

            let _mess:&str = $fmt_string; // make sure that a &str is passed

        }

        send_log(validate_msg($fmt_string, &[$($arg),*]).as_str(),FATAL);

            
    }
    //
}
//
//
#[macro_export]
macro_rules! CERROR {
    //
    

    ($fmt_string:expr) => {

        use crate::Core::logger::send_log;
        {

            let _mess:&str = $fmt_string; // make sure that a &str is passed

        }

        send_log($fmt_string,crate::Core::logger::Level::ERROR);


    };
    //
    //
    ($fmt_string:expr, $( $arg:expr ),*) => {

     
        {
            use crate::Core::logger::{send_log,validate_msg,Level::ERROR};
            let _mess:&str = $fmt_string; // make sure that a &str is passed

            send_log(validate_msg($fmt_string, &[$($arg),*]).as_str(),ERROR);

        }


            
    };
    //
}
//
//
#[macro_export]
macro_rules! CWARN {
    //
    ($fmt_string:expr) => {

        use crate::Core::logger::{send_log,Level::WARN};


        {

            let _mess:&str = $fmt_string; // make sure that a &str is passed

        }

        send_log($fmt_string,WARN);


    };
    //
    //
    ($fmt_string:expr, $( $arg:expr ),*) => {

        use crate::Core::logger::{send_log,validate_msg,Level::WARN};

        {

            let _mess:&str = $fmt_string; // make sure that a &str is passed

        }

        send_log(validate_msg($fmt_string, &[$($arg),*]).as_str(),WARN);

            
    }
    //
}
//
//
#[macro_export]
macro_rules! CINFO {
    //
    ($fmt_string:expr) => {

        use crate::Core::logger::{send_log,Level::INFO};

        {

            let _mess:&str = $fmt_string; // make sure that a &str is passed

        }

        send_log($fmt_string,INFO);


    };
    //
    //
    ($fmt_string:expr, $( $arg:expr ),*) => {


        use crate::Core::logger::{send_log,validate_msg,Level::INFO};


        {

            let _mess:&str = $fmt_string; // make sure that a &str is passed

        }

        send_log(validate_msg($fmt_string, &[$($arg),*]).as_str(),INFO);

            
    }
    //
}
//
//
#[macro_export]
macro_rules! CDEBUG {
    //

    ($fmt_string:expr) => {

        use crate::Core::logger::{send_log,validate_msg,Level::DEBUG};

        {

            let _mess:&str = $fmt_string; // make sure that a &str is passed

        }

        send_log($fmt_string,DEBUG);


    };
    //
    //
    ($fmt_string:expr, $( $arg:expr ),*) => {

        use crate::Core::logger::{send_log,validate_msg,Level::DEBUG};


        {

            let _mess:&str = $fmt_string; // make sure that a &str is passed

        }

        send_log(validate_msg($fmt_string, &[$($arg),*]).as_str(),DEBUG);

            
    }
    //
}
//
//
#[macro_export]
macro_rules! CTRACE {
    //
    ($fmt_string:expr) => {

        use crate::Core::logger::{send_log,validate_msg,Level::CTRACE};


        {

            let _mess:&str = $fmt_string; // make sure that a &str is passed

        }

        send_log($fmt_string,TRACE);


    };
    //
    //
    ($fmt_string:expr, $( $arg:expr ),*) => {


        use crate::Core::logger::{send_log,validate_msg,Level::CTRACE};

        {

            let _mess:&str = $fmt_string; // make sure that a &str is passed

        }

        send_log(validate_msg($fmt_string, &[$($arg),*]).as_str(),TRACE);

            
    }
    //
}
//
//
//
pub fn send_log(msg: &str,lvl:Level) {


    match lvl {
        //
        // cant supress FATAL and ERROR log
        //
        Level::FATAL => {

            let log = Log::new(Level::FATAL,msg.red());
            push_log(log);
            shutdown_logging(FAILURE);


        },
        //
        Level::ERROR => {

            let log = Log::new(Level::ERROR,msg.red());
            push_log(log);


        },
        //
        Level::WARN => {

            if LOG_WARN_ENABLE.load(Ordering::Relaxed) == 1 {

                let log = Log::new(Level::WARN,msg.yellow());
        
                push_log(log);
        
            }


        },
        //
        Level::INFO => {

            if LOG_INFO_ENABLE.load(Ordering::Relaxed) == 1 {

                let log = Log::new(Level::INFO,msg.blue());
                push_log(log);
        
            }

        },
        //
        Level::DEBUG => {


            if LOG_DEBUG_ENABLE.load(Ordering::Relaxed) == 1 {

            let log = Log::new(Level::DEBUG,msg.green());
            push_log(log);

            }


        }
        //
        Level::TRACE => {


            if LOG_TRACE_ENABLE.load(Ordering::Relaxed) == 1 {

                let log = Log::new(Level::DEBUG,msg.green());
                push_log(log);

            }

        }
        //
        //
    }

}
//
//
pub fn disable_log_lvl(level: Level) {

    if (&level as *const Level as usize) < 2 {
        CERROR!("Cant disable logging for Error and Fatal");
    }

    match level {
        Level::WARN =>  LOG_WARN_ENABLE.store( 0,Ordering::Release),
        Level::INFO =>  LOG_INFO_ENABLE.store( 0,Ordering::Release),
        Level::DEBUG => {

            if CRELEASE == 1 { CERROR!("Cant disable logging for Debug in Release mode"); }
            else { LOG_DEBUG_ENABLE.store( 0,Ordering::Release) }

        },
        Level::TRACE => {

            if CRELEASE == 1 { CERROR!("Cant disable logging for Trace in Release mode"); }
            else { LOG_TRACE_ENABLE.store( 0,Ordering::Release) }

        },
        _ => (),
    }
}
//
//
pub fn assertions_failures(expr: &str,message: &str,file:&str,line:u32) {


    is_init();
    
    let msg = format!("ASSERTION FAILED: {} in file {} at line {} Reason: {} ",expr ,file,line,message);

    CFATAL!(msg.as_str());
    

} 
//
//
pub fn validate_msg(msg: &str,args:&[&str]) -> String {

    let msg_sliced = slice_brackets_str(&msg);
  
    let nb_brackets = get_nb_brackets(&msg_sliced);

    if args.len() > nb_brackets as usize {

        CFATAL!(
            "the message:[ {} ] contains {} format bracket(s) but you have passed {} arguments ",
            msg,
            format!("{}",nb_brackets.clone()).as_str(),
            format!("{}",&args.len()).as_str()
        
        );

    }

    let mut iter_brk:usize = 0;
    let mut f_msg = String::new();

    for slice in msg_sliced.iter() {
        
        if slice != &"{}" {

            f_msg = format!("{}{}",f_msg,slice);

        }   

        else {

            if args.len() - 1 >= iter_brk as usize {
       
                f_msg = format!("{}{}",f_msg,args[iter_brk]);

                iter_brk += 1;
            }
             

        }

    }

    f_msg


}