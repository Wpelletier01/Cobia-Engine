
use std::sync::Mutex;
use std::time::{Instant,Duration};

use crate::renderer::RenderingSys;
use super::logs::{CTRACE,init,CINFO,CDEBUG,CFATAL};

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
// Engine
//
pub struct Engine {

    rendering_sys: RenderingSys

}
//
impl Engine {

    pub fn init(
        application_name:       &str,
        application_version:    (u32,u32,u32),
        win_width:              u16,
        win_height:             u16) ->  Self {
        //
        // init the log system
        init().unwrap();

        CINFO("Logging system initialized successfully");

        CTRACE("Start engine initialisation");

        CTRACE("Start the rendering system initialisation");
        let mut rendering_sys = RenderingSys::new(application_name,application_version).unwrap();

        rendering_sys.set_window_title(application_name).unwrap();
        rendering_sys.set_window_size(win_width,win_height).unwrap();

        CINFO("Rendering system initialized successfully");

        CINFO("Engine initialisation done");

        Self { rendering_sys }

    }
    //
    fn close(&self) {

        // TODO: thing to stop should be there


    }
    //
}