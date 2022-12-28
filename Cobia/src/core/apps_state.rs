
use std::sync::{Mutex,MutexGuard};
use std::time::{Instant,Duration};

lazy_static::lazy_static!{ 

    static ref APPLICATION: Mutex<StateMaster> = Mutex::new(StateMaster::init());

}

/// Store information for the application that use the engine
struct StateMaster {

    start_time: Instant
    
}
//
impl StateMaster {
    //
    fn init() -> Self { StateMaster { start_time: Instant::now()}}
    //
}
//
//
/// get Duration since the Engine initialized 
pub(crate) fn get_prog_elapsed_time() -> Duration { 
    
    match APPLICATION.lock() {
        Ok(app) =>  return app.start_time.elapsed(),
        Err(e) => {
            
            // TODO: add logging for indicating failing
            return Duration::new(0, 0);

        }
    }
}
//
//