
// TODO: add comment
// TODO: clean log message


use super::{EVlkApi,Result};
use crate::core::logs::CVLK;

use std::sync::Arc;

use vulkano::{
    instance::{
        Instance,
        debug::{

            DebugUtilsMessenger,
            DebugUtilsMessengerCreateInfo,
            DebugUtilsMessageSeverity,
            DebugUtilsMessageType,
            Message

        }


    },


};


pub(crate) fn init_debug_utils(inst:Arc<Instance>) -> Result<DebugUtilsMessenger,EVlkApi> {

    let create_info = DebugUtilsMessengerCreateInfo {

        message_severity: DebugUtilsMessageSeverity {
            warning: true,
            error:   true,
            verbose: true,
            information: true,
            ..Default::default()
        },
        message_type: DebugUtilsMessageType  {
            general: true,
            validation: true,
            performance: true,
            ..Default::default()

        },
        ..DebugUtilsMessengerCreateInfo::user_callback(Arc::new(|msg|log_callback(msg)))

    };



    Ok(
        unsafe {
            DebugUtilsMessenger::new(inst, create_info).map_err(|e|
                EVlkApi::DEBUG.attach_printable_default(e)
            )?
        
        }
    )
    
}


fn log_callback(message:&Message) {

    let severity =
        if message.severity.error {
            " [ERROR]"
        } else if message.severity.warning {
            " [WARN]"
        } else if message.severity.verbose {
            " [VERBOSE]"
        } else if message.severity.information {
            " [INFO]"
        } else {
            " [UNKNOWN]"
        };

    let type_ =
        if message.ty.general {
            " [GENERAL]"
        } else if message.ty.performance {
            " [PERFORM]"
        } else if message.ty.validation {
            " [VALIDATION]"
        } else {
          " [UNKNOWN]"
        };

    
    let log = format!(
        "[{}] {}{}\n                       {}",
        message.layer_prefix.unwrap_or("UNKNOWN") ,
        severity,
        type_,
        message.description
    );
    
    CVLK(&log);
    
}


