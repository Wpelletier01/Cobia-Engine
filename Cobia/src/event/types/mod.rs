// TODO: add comment

pub mod input;
pub mod window;

use window::WindowEvent;
use input::{KeyboardEvent,TextInputEvent,MouseEvent,ModifierChangeEvent};

use std::fmt::{Debug, Display, Formatter, Result, write};


pub(crate) enum CEvent {
    
    Any,
    Ignored,
    Unknown,
    Window(WindowEvent),
    Keyboard(KeyboardEvent),
    Mouse(MouseEvent),
    ModifierStateChange(ModifierChangeEvent),
    Suspended,
    Resumed,
    // renderer system will do some stuff
    RedrawRequest,
    RedrawClear,
    ReceivedChar(char),
    TextInput(TextInputEvent),
    TextInputModeEnabled,
    TextInputModeDisabled

    // TODO: add other

}
//
impl Display for CEvent {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result {

        match self {

            Self::Any =>
                write!(f,"Any"),
            Self::Ignored =>
                write!(f,"Ignored"),
            Self::Unknown =>
                write!(f,"Unknown"),
            Self::Window(win) =>
                write!(f,"Window: {}",win),
            Self::Keyboard(key) =>
                write!(f,"Keyboard: Key: {} Action: {}",key.key,key.action),
            Self::Mouse(event) =>
                write!(f,"Mouse: {}",event),
            Self::ModifierStateChange(event) =>
                write!(f,"Modifier state: {}",event),
            Self::Suspended =>
                write!(f,"Os Suspend"),
            Self::Resumed =>
                write!(f,"Os Resumed"),
            Self::RedrawRequest =>
                write!(f,"Redraw Request"),
            Self::RedrawClear =>
                write!(f,"Redraw Clear"),
            Self::ReceivedChar(c) =>
                write!(f,"Receive char : {}",c),
            Self::TextInput(t) =>
                write!(f,"{}",t),
            Self::TextInputModeEnabled =>
                write!(f,"Text input mode enabled"),
            Self::TextInputModeDisabled =>
                write!(f,"Text input mode disabled")


        }
    }


}


