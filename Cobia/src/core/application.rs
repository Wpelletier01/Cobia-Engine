
use std::sync::Mutex;
use std::time::{Instant,Duration};



// ------------------------------------------------------------------------------------------------
// Clock information
//
//
lazy_static::lazy_static! {

    static ref APPS_CLOCK:  Mutex<Instant> = Mutex::new(Instant::now());

}


/// Return the elapsed time since the program started
pub fn get_prog_elapsed_time() -> Duration {
    //
    match APPS_CLOCK.lock() {

        Ok(clock) => clock.elapsed(),
        Err(e) => {

            eprintln!("unable to access the engine's clock because: {}", e.to_string());
            eprintln!("a duration value of 0 will be returned");

            Duration::new(0, 0)

        }

    }
    //
}
//
// ------------------------------------------------------------------------------------------------
// Host information
//
// TODO: continue
// 
/// Info about the host that the engine is running on 
struct HostInfo {

    os: String,

}
//
impl HostInfo {
    //
    /// Gathering information on the host
    pub fn new() -> Self {

        #[cfg(target_os = "linux")]
        let os = "linux".to_string();

        #[cfg(target_os = "windows")]
        let os = "windows".to_string();


        HostInfo { 
        
            os: os  
        
        
        }


    }
    //
    // 
}
//
//
// ------------------------------------------------------------------------------------------------