// TODO: add comment

pub mod input;
pub mod window;

use window::WindowEvent;
use input::{KeyboardEvent,TextInputEvent,MouseEvent,ModifierChangeEvent};



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


