#![allow(dead_code)]

use crate::define::{CRELEASE,FMIN_AS_SECONDS,FHOUR_AS_SECONDS};
use super::application::get_prog_elapsed_time;

use std::sync::Mutex;
use std::time::Duration;
use std::env;

use colored::{Colorize,ColoredString};

//
//
// TODO: add function to write the logs to a file  
//
//
// ------------------------------------------------------------------------------------------------
// shouldn't occur a lot in this engine but this global variable is needed 
//
lazy_static::lazy_static! {
    //
    static ref LOG_SUBSYSTEM: Mutex<LogSystem> = Mutex::new(
        LogSystem { 
            queue:      LogQueue::new(), 
            init:       false, 
            debug_log:  true,
            info_log:   true,
            warn_log:   true, 
            trace_log:  true 
        }
    );
    //
}
//
// ------------------------------------------------------------------------------------------------
// Constant 
// 
const       LOG_BUFFER_SIZE:    usize       = 20000;
const       LOG_MAX_QUEUE_SIZE: usize       = 300;
const       MAX_LINE_LEN:       usize       = 100;
const       CFAILURE:           u8          = 0;
const       CSUCCESS:           u8          = 1;
const       LEVEL_STRING:       [&str;6]    = [
    "[FATAL]:","[ERROR]:","[WARN]: ","[INFO]: ", "[DEBUG]:","[TRACE]:"
];
// tab jump for if a log have multiple lines, they start all at the same position
const TAB_MESSAGE: &str = "\n                       "; // 23 columns of whitespace
//
//
// ------------------------------------------------------------------------------------------------
// The log subsystem
// 
// TODO: create functions for disable/enable logging
//
/// creates and sends a log messages through out the Engine
pub(crate) struct LogSystem {

    queue:      LogQueue,
    init:       bool,
    debug_log:  bool,
    info_log:   bool,
    warn_log:   bool,
    trace_log:  bool,

}
//
impl LogSystem {
    //
    /// Initialize the log subsystem
    pub(crate) fn initialize(&mut self) { 
        //
        // they are all enable by default
        //
        // check if in release mode and if so disable debug and trace logging
        if CRELEASE == 1{

            self.debug_log = false;
            self.info_log  = false;

        }
        //
        self.init = true;
        // add the initialize log
        crate::CTRACE!("Start the log subsystem");
        //
        //    
    }
    //
    //
    /// Add a log entry to the end of the LOG_QUEUE  
    /// 
    /// # Parameters
    /// 
    /// * log - a log entry to be added to the queue
    /// 
    fn push_log(&mut self,level: Level,msg: &str) { 
        //
        // check to make sure that the log subsystem is initialized
        if !self.is_init(){
            // TODO: found a better solution because the log subsystem
            //       should not make the application crash
            panic!("try to add a log but the log subsystem is not initialized");
            //
        }
        //
        // check if the level is enabled and then pass it to the queue
        match level {
            //
            Level::DEBUG => {

                if self.debug_log {

                    self.queue.push(Log::new(level, msg.green()));

                }

            },
            //
            Level::INFO => {

                if self.info_log {

                    self.queue.push(Log::new(level, msg.blue()));

                }

            },
            //
            Level::TRACE => {

                if self.trace_log {

                    self.queue.push(Log::new(level, msg.magenta()));

                }

            },
            //
            Level::WARN => {

                if self.warn_log {

                    self.queue.push(Log::new(level,msg.yellow()));

                }

            },
            //
            // Fatal and Error types are not allowed to be disabled so no need to be checked
            _ => self.queue.push(Log::new(level,msg.red()))
            //
            //
        }
        //
        self.print();
        //
    }
    //
    /// wrapper for the macro println! but only for the
    /// log message this function will probably be unable to be used by default
    fn print(&self) { 
        //
        println!("{}", match self.queue.content.last(){

            Some(v) => v.as_string(),
            None => "".to_string(),

        }); 
        //
    }
    //
    /// Check if the sub system logging have been initialize
    fn is_init(&self) -> bool { self.init }
    //
    //
}
//
//
/// Drop the log system
pub fn end() {
    //
    match LOG_SUBSYSTEM.lock() {

        Ok(sys) => drop(sys),
        Err(err) => {

            // TODO: found solution to this situation
            println!("unable to destroy log system because: {}",err.to_string())

        }

    }
    

}
//
//
/// Initialize the log subsystem 
pub fn init() {
    //
    match LOG_SUBSYSTEM.lock() {
        
        Ok(mut sys) => sys.initialize(),
        
        Err(err) => {

            // TODO: found solution to this situation
            println!("unable to destroy log system because: {}",err.to_string())

        }
        
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
pub(crate) fn fmt_log(level: Level, message: String) -> String{
    //
    // level represent the index in the CLEVEL_STRING
    // " {TIME} {TYPE} {MESSAGE}"
    let msg = format!(
        "{} {} {}",
        fmt_duration_log(get_prog_elapsed_time()),
        LEVEL_STRING[level as usize],
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
/// parse an unsigned integer to a string and if it is a single digit number, it add a zero 
/// before it
/// 
/// # Arguments
/// 
/// * 'value' - a unsigned integer to parse into a string
/// 
fn format_single_digit_value(value: u32) -> String {
    //
    if value < 9 { 
        return format!("0{}",value); 
    }
    
    format!("{}",value)
    //
}
//
// ------------------------------------------------------------------------------------------------
// Log Struct 
//
/// Vector that store every log entry
pub struct LogQueue { content: Vec<Log> }
//
impl LogQueue {
    //
    /// initialize the queue
    fn new() -> Self {
        //
        let q :Vec<Log> = Vec::with_capacity(LOG_MAX_QUEUE_SIZE);
        
        LogQueue{ content: q}
        //
    }
    //
    /// add a log entry to the end queue
    /// 
    /// # Arguments
    ///  
    /// * 'log' - a Log entry to be add
    /// 
    fn push(&mut self,log:Log) {
        //
        if self.content.len() + 1 == LOG_MAX_QUEUE_SIZE {
            
            //TODO: implement something for remedy this situation
            //      I just dont have idea for now
            unimplemented!();
            
        }
        
        self.content.push(log);
        //
    }
    //
    //
}
//
//
/// Store a log entry 
pub(crate) struct Log{ content:String }
//
impl Log{
    //
    /// initialize a new log entry
    /// 
    /// # Arguments
    /// 
    /// * 'level'   - type of log entry
    /// * 'message' - colored message that the log entry should show
    /// 
    pub(crate) fn new(level:Level, message:ColoredString) -> Self {
        //
        // format version message 
        #[allow(unused_assignments)]
        let mut fmt_msg = String::new();
        //
        if message.len() > MAX_LINE_LEN {
            //
            // get the message color and if none set default one (white) 
            let msg_color = match message.fgcolor() { 

                Some(color) => color,
                None => colored::Color::White

            }; 
            //
            // how many lines the message should have
            let nb_lines = message.len() as f32 / MAX_LINE_LEN as f32;
            //
            // will store each line
            let mut lines: Vec<String> = Vec::new();
            //
            //
            for line in 0..nb_lines.ceil() as i32 {
                //
                // store the 100 range of characters depending on the value of 'line'
                lines.push(message[line as usize*MAX_LINE_LEN..].to_string());
                //
            }
            // align the multiple lines together
            fmt_msg = lines.join(TAB_MESSAGE).color(msg_color).to_string();
            //
            //
        } else{
        
            fmt_msg = message.to_string();

        }
        //
        //
        // add the header to the message  
        let mut msg = fmt_log(level,fmt_msg);
        //
        // check if the len of the message is bigger than the max allowed
        if msg.len() > LOG_BUFFER_SIZE - 1 {

            msg = msg[..LOG_BUFFER_SIZE].to_string();

        }

        Log{ content: msg}
        //
        //
    }
    //
    /// return the log as a string
    pub fn as_string(&self) -> String { self.content.to_string() }
    //
    //
}
//
//
// ------------------------------------------------------------------------------------------------
// Level enum
//
#[repr(usize)]
#[derive(PartialEq)]
/// represents the index of the type of log in the const LEVEL_STRING
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
// ------------------------------------------------------------------------------------------------
// Macro declaration
//
/// boilerplate code for the logger macros
pub(crate) fn push_log(msg:&str,lvl:Level) {

    match LOG_SUBSYSTEM.lock() {

        Ok(mut sys) => sys.push_log(lvl, msg),
        Err(_) => {

            //TODO: find a solution to this situation
        }

    }
    //
}
//
//
/// Slice the given string where brackets couple are founded
/// 
/// # Arguments
/// 
/// * 'msg' - the message to be slice
/// 
fn slice_brackets_str(msg:&str) -> (Vec<String>,usize){

    let mut slices:Vec<String> = Vec::new();

    // iterator index through all the characters of the message
    let mut f_index:usize = 0;
    // iterator over how many {} founded 
    let mut founding:usize = 0;

    for (index,c) in msg.chars().enumerate() {

        // if we found this '{}' we slice the message there 
        if c == '{' && msg.chars().nth(index + 1) == Some('}') {

                let slice = &msg[f_index .. index];

                // add the left side of the {} founded 
                slices.push(slice.to_string());
                // add {} to be able to replaced them with arguments
                slices.push("{}".to_string());

                // then we jump over {}
                f_index = index + 2;

                founding += 1;
            

        }
    }
    // get the last slice and add it with the others
    let slice_end = &msg[f_index .. msg.chars().count()];
    
    slices.push(slice_end.to_string());
    //
    (slices,founding)
    //
    //
}
//
//
/// validate that the string passed to the macro is a valid string with 
/// valid number of arguments
/// 
/// # Arguments
/// 
/// * 'msg' - the string message to be validated
/// * 'args' - the arguments to be validated and passed to the message
pub fn validate_msg(msg: &str,args:&[&str]) -> String {
    //
    let (msg_sliced,nb_brackets) = slice_brackets_str(&msg);
    //
    //    
    // validate that the number of arguments passed are the same as the number of
    // couple brackets
    if args.len() > nb_brackets {

        crate::CFATAL!(
            "the message:[ {} ] contains {} format bracket(s) but you have passed {} arguments ",
            msg,
            format!("{}",nb_brackets.clone()).as_str(),
            format!("{}",&args.len()).as_str()
        
        );

    }
    //
    // replace each bracket by the arguments
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
    //
    f_msg
    //
    //
}
//
//
#[macro_export]
macro_rules! CFATAL {
    //
    //
    ($fmt_string:expr) => {

        use crate::core::logs::{push_log,Level::FATAL};
        
        push_log($fmt_string,FATAL);


    };
    //
    ($fmt_string:expr, $( $arg:expr ),*) => {
        
        use crate::core::logs::{push_log,validate_msg,Level::FATAL};
      
        push_log(validate_msg($fmt_string, &[$($arg),*]).as_str(),FATAL);

            
    }
    //
}
//
//
#[macro_export]
macro_rules! CERROR {
    //
    //
    ($fmt_string:expr) => {
        //
        use crate::core::logs::{Level::ERROR,push_log};
   
        push_log($fmt_string,Level::ERROR);


    };
    //
    ($fmt_string:expr, $( $arg:expr ),*) => {
        //
        use crate::core::logs::{push_log,validate_msg,Level::ERROR};
            
        push_log(validate_msg($fmt_string, &[$($arg),*]).as_str(),ERROR);

            
    };
    //
}
//
//
#[macro_export]
macro_rules! CWARN {
    //
    ($fmt_string:expr) => {

        use crate::Core::logs::{push_log,Level::WARN};

        push_log($fmt_string,WARN);


    };
    //
    ($fmt_string:expr, $( $arg:expr ),*) => {

        use crate::Core::logger::{send_log,validate_msg,Level::WARN};

        push_log(validate_msg($fmt_string, &[$($arg),*]).as_str(),WARN);

            
    }
    //
}
//
//
#[macro_export]
macro_rules! CINFO {
    //
    ($fmt_string:expr) => {

        use crate::core::logs::{send_log,Level::INFO};

        push_log($fmt_string,INFO);


    };
    //
    ($fmt_string:expr, $( $arg:expr ),*) => {

        use crate::core::logs::{push_log,validate_msg,Level::INFO};

        push_log(validate_msg($fmt_string, &[$($arg),*]).as_str(),INFO);

            
    }
    //
}
//
//
#[macro_export]
macro_rules! CDEBUG {
    //
    ($fmt_string:expr) => {

        use crate::core::logs::{push_log,validate_msg,Level::DEBUG};

        push_log($fmt_string,DEBUG);

    };
    //
    ($fmt_string:expr, $( $arg:expr ),*) => {

        use crate::core::logs::{push_log,validate_msg,Level::DEBUG};

        push_log(validate_msg($fmt_string, &[$($arg),*]).as_str(),DEBUG);
       
    }
    //
}
//
//
#[macro_export]
macro_rules! CTRACE {
    //
    ($fmt_string:expr) => {

        use crate::core::logs::{push_log,Level::TRACE};

        push_log($fmt_string,TRACE);

    };
    //
    ($fmt_string:expr, $( $arg:expr ),*) => {

        use crate::core::logs::{push_log,validate_msg,Level::CTRACE};

        push_log(validate_msg($fmt_string, &[$($arg),*]).as_str(),TRACE);

            
    }
    //
}
//
//
// ------------------------------------------------------------------------------------------------