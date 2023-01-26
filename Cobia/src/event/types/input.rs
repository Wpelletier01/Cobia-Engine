// TODO: add comment

use super::CEvent;

use std::fmt::{Display, Formatter, Result, write};

//
//
// ------------------------------------------------------------------------------------------------
//
//
pub enum StateAction {

    Press,
    Release,

}
//
impl Display for StateAction {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {

            Self::Press => write!(f,"Press"),
            Self::Release => write!(f,"Release")

        }
    }

}
//
//
// ------------------------------------------------------------------------------------------------
// Modifier
//
pub const NULL_MOD:                 u8 = 0;
pub const CONTROL:                  u8 = 1;
pub const SHIFT:                    u8 = 2;
pub const ALT:                      u8 = 3;
pub const SUPER:                    u8 = 4;
pub const CONTROL_ALT:              u8 = 5;
pub const CONTROL_SHIFT:            u8 = 6;
pub const SHIFT_ALT:                u8 = 7;
pub const CONTROL_ALT_SHIFT:        u8 = 8;
//
// copy from winit::event::ModifiersState
pub(crate) struct ModifierStateKeeper {

    current: u8

}
//
impl ModifierStateKeeper {

    pub(crate) fn init() -> Self { Self { current: NULL_MOD } }
    pub(crate) fn no_modifier(&self)    -> bool { self.current == NULL_MOD }
    pub(crate) fn ctrl(&self)           -> bool { self.current == CONTROL }
    pub(crate) fn superk(&self)         -> bool { self.current == SUPER }
    pub(crate) fn shift(&self)          -> bool { self.current == SHIFT }
    pub(crate) fn alt(&self)            -> bool { self.current == ALT }
    pub(crate) fn ctrl_alt(&self)       -> bool { self.current == CONTROL_ALT }
    pub(crate) fn ctrl_shift(&self)     -> bool { self.current == CONTROL_SHIFT }
    pub(crate) fn shift_alt(&self)      -> bool { self.current == SHIFT_ALT }
    pub(crate) fn ctrl_alt_shift(&self) -> bool { self.current == CONTROL_ALT_SHIFT }

}
//
pub enum ModifierKey {

    Shift,
    Ctrl,
    Alt,
    Super,
    Null

}
//
pub struct ModifierChangeEvent {

    change_value: u8

}
//
impl ModifierChangeEvent {

    pub fn new(change_value:u8) -> Self { Self { change_value} }

}
//
impl Display for ModifierChangeEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
       
        let flag = match self.change_value {
           
            NULL_MOD => "No Mod enable",    
            CONTROL => "Control",          
            SHIFT => "Shift",            
            ALT => "Alt",             
            SUPER => "Super",     
            CONTROL_ALT => "Control Alt",      
            CONTROL_SHIFT => "Control Shift",
            SHIFT_ALT => "Shift Alt",
            CONTROL_ALT_SHIFT => "Control Alt Shift",  
            _ => "Unknown"
         
        };
        
        write!(f,"{}",flag)
        
    }   
    
    
}
//
//
// ------------------------------------------------------------------------------------------------
// Mouse Event
//

pub enum MouseEvent {

    MouseButton(MouseButtonEvent),
    MouseMovement(MouseMovedEvent),
    MouseEscape,
    MouseEntered,
    MouseWheelRoll(MouseWheelRollEvent)

}
//
impl Display for MouseEvent {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result {

        match self {

            Self::MouseButton(event) =>
                write!(f,"Button: {} Action: {}",event.button,event.action),
            Self::MouseMovement(event) =>
                write!(f,"Mouse is at x: {} px y: {} ",event.x,event.y),
            Self::MouseEscape =>
                write!(f,"Escaped the window surface"),
            Self::MouseEntered =>
                write!(f,"Entered the window surface"),
            Self::MouseWheelRoll(event) =>
                write!(f,"Wheel '{}' been scroll to colomn: {} row: {}",event.phase,event.column,event
                    .row)


        }

    }


}
//
//
pub enum MouseButton {

    Left,
    Right,
    Wheel,
    Unknown

}
//
impl Display for MouseButton {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {

        match self {

            Self::Left => write!(f,"Left"),
            Self::Right => write!(f,"Right"),
            Self::Wheel => write!(f,"Wheel"),
            Self::Unknown => write!(f,"Unknown")

        }

    }

}
//
pub struct MouseButtonEvent {

    button: MouseButton,
    action: StateAction

}
//
impl MouseButtonEvent {

    pub fn new(button:MouseButton,action:StateAction) -> Self { Self { button, action } }

}
//
pub enum PhaseWheelScroll {

    Started,
    Moved,
    Stopped,
    Canceled

}
//
impl Display for PhaseWheelScroll {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {

            PhaseWheelScroll::Started =>
                write!(f,"Started"),
            PhaseWheelScroll::Moved =>
                write!(f,"Moved"),
            PhaseWheelScroll::Stopped =>
                write!(f,"Stopped"),
            PhaseWheelScroll::Canceled =>
                write!(f,"Canceled")

        }
    }
}
//
pub struct MouseWheelRollEvent {

    phase:      PhaseWheelScroll,
    column:     f32,
    row:        f32

}
//
impl MouseWheelRollEvent {

    pub fn new(phase:PhaseWheelScroll,column:f32,row:f32) -> Self { Self { phase, column, row }}

}
//
pub struct MouseMovedEvent {

    x: f64,
    y: f64,

}
//
impl MouseMovedEvent {

    pub fn new(x:f64,y:f64) -> Self { Self { x, y }}

}
//
//
// ------------------------------------------------------------------------------------------------
// Text Input
//
pub struct TextInputEvent {

    pub content: String,
    pub position:  Option<(usize,usize)>

}
//
impl TextInputEvent {

    pub fn new(content:String,position:Option<(usize,usize)>) -> Self { Self { content, position } }

}
//
impl Display for TextInputEvent {
    
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        
        match self.position {
            
            Some(pos) => 
                write!(f," Text Input at column: {} row: {} content: {}",pos.0,pos.1,self.content),
            None =>
                write!(f,"Text Input at unknown position content: {}",self.content)
            
            
        }
        
    }
    
}
//
//
// ------------------------------------------------------------------------------------------------
// Keyboard
//
pub struct KeyboardEvent {

    pub key:        Key,
    pub action:     StateAction,

}
//
impl KeyboardEvent {

    pub fn new(key:Key,action:StateAction) -> Self { Self { key, action} }

}
//
pub enum Key {
    //
    // Functionality
    CapsLock,
    Space,
    Tab,
    Enter,
    Return,
    Escape,
    BackSpace,
    // cursor control
    ScrollLock,
    Delete,
    Insert,
    Home,
    End,
    PgUp,
    PgDn,
    Up,
    Down,
    Left,
    Right,
    // numpad
    NpLock,
    NpDiv,
    NpMult,
    NpAdd,
    NpSub,
    NpEnter,
    // on
    Np0,
    Np1,
    Np2,
    Np3,
    Np4,
    Np5,
    Np6,
    Np7,
    Np8,
    Np9,
    NpDot,
    // off
    NpInsert,
    NpEnd,
    NpDown,
    NpPgUp,
    NpPgDn,
    NpLeft,
    NpRight,
    NpHome,
    NpUp,
    NpDelete,
    // NpBegin,
    // function key
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    // Modifier
    LSuper,
    RSuper,
    LCtrl,
    RCtrl,
    LShift,
    RShift,
    LAlt,
    RAlt,
    // multimedia key
    BrwsBack,
    BrwsForward,
    BrwsRefresh,
    BrwsStop,
    BrwsSearch,
    BrwsFavorite,
    BrwsHome,
    VlmMute,
    VlmDown,
    VlmUp,
    MediaNext,
    MediaPrev,
    MediaStop,
    MediaPlay,
    LaunchMail,
    LaunchMedia,
    LaunchApp1,
    LaunchApp2,
    // Other
    AppsKey,
    PrintScreen,
    CtrlBreak,
    Pause,
    Break,
    Help,
    Sleep,
    Menu,
    Scroll,
    Capital,
    // number
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    // Letter
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    // symbol
    Accent,
    Tilde, // ~
    Exclamation,
    At,
    Hash,
    DollarSign,
    Percent,
    And,
    Astrix,
    OpenParentheses,
    CloseParentheses,
    Underscore,
    Minus,
    Plus,
    Equal,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    VerticalPipe,
    Backslash,
    Colon,
    Semicolon,
    QuotationMarks,
    Apostrophe,
    Comma,
    Less,
    Greater,
    Period,
    Slash,
    QuestionMark,
    Grave,
    //
    Unknown
    //
}
//
impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {

        let k = match self {

            Self::A =>                  "A",
            Self::B =>                  "B",
            Self::C =>                  "C",
            Self::D =>                  "D",
            Self::E =>                  "E",
            Self::F =>                  "F",
            Self::G =>                  "G",
            Self::H =>                  "H",
            Self::I =>                  "I",
            Self::J =>                  "J",
            Self::K =>                  "K",
            Self::L =>                  "L",
            Self::M =>                  "M",
            Self::N =>                  "N",
            Self::O =>                  "O",
            Self::P =>                  "P",
            Self::Q =>                  "Q",
            Self::R =>                  "R",
            Self::S =>                  "S",
            Self::T =>                  "T",
            Self::U =>                  "U",
            Self::V =>                  "V",
            Self::W =>                  "W",
            Self::X =>                  "X",
            Self::Y =>                  "Y",
            Self::Z =>                  "Z",
                                        
            Self::N0 =>                 "0",
            Self::N1 =>                 "1",
            Self::N2 =>                 "2",
            Self::N3 =>                 "3",
            Self::N4 =>                 "4",
            Self::N5 =>                 "5",
            Self::N6 =>                 "6",
            Self::N7 =>                 "7",
            Self::N8 =>                 "8",
            Self::N9 =>                 "9",
                                        
            Self::CapsLock =>           "CapsLock",
            Self::Space =>              "Space",
            Self::Tab =>                "Tab",
            Self::Enter =>              "Enter",
            Self::Return =>             "Return",
            Self::Escape =>             "Escape",
            Self::BackSpace =>          "BackSpace",
            Self::Delete =>             "Delete",
            Self::Insert =>             "Insert",
            Self::Home =>               "Home",
            Self::End =>                "End",
            Self::PgUp =>               "Page Up",
            Self::PgDn =>               "Page Down",
            Self::Up =>                 "Up",
            Self::Down =>               "Down",
            Self::Left =>               "Left",
            Self::Right =>              "Right",
            Self::ScrollLock =>         "Scroll Lock",
                                        
            Self::NpLock =>             "NumPad Lock",
            Self::NpDiv =>              "NumPad Divide",
            Self::NpMult =>             "NumPad Multiple",
            Self::NpAdd =>              "NumPad Add",
            Self::NpSub =>              "NumPad Subtract",
            Self::NpEnter =>            "NumPad Enter",
            Self::Np0 =>                "NumPad 0",
            Self::Np1 =>                "NumPad 1",
            Self::Np2 =>                "NumPad 2",
            Self::Np3 =>                "NumPad 3",
            Self::Np4 =>                "NumPad 4",
            Self::Np5 =>                "NumPad 5",
            Self::Np6 =>                "NumPad 6",
            Self::Np7 =>                "NumPad 7",
            Self::Np8 =>                "NumPad 8",
            Self::Np9 =>                "NumPad 9",
            Self::NpDot =>              "NumPad Dot",
            Self::NpInsert =>           "NumPad Insert",
            Self::NpEnd =>              "NumPad End",
            Self::NpDown =>             "NumPad Down",
            Self::NpPgUp =>             "NumPad Page Up",
            Self::NpPgDn =>             "NumPad Page Down",
            Self::NpLeft =>             "NumPad Left",
            Self::NpRight =>            "NumPad Right",
            Self::NpHome =>             "NumPad Home",
            Self::NpUp =>               "NumPad Up",
            Self::NpDelete =>           "NumPad Delete",
                                        
            Self::F1 =>                 "F1",
            Self::F2 =>                 "F2",
            Self::F3 =>                 "F3",
            Self::F4 =>                 "F4",
            Self::F5 =>                 "F5",
            Self::F6 =>                 "F6",
            Self::F7 =>                 "F7",
            Self::F8 =>                 "F8",
            Self::F9 =>                 "F9",
            Self::F10 =>                "F10",
            Self::F11 =>                "F11",
            Self::F12 =>                "F12",
                                        
            Self::LSuper =>             "Left Super",
            Self::RSuper =>             "Right Super",
            Self::LCtrl =>              "Left Control",
            Self::RCtrl =>              "Right Control",
            Self::LShift =>             "Left Shift",
            Self::RShift =>             "Right Shift",
            Self::LAlt =>               "Left Alt",
            Self::RAlt =>               "Right Alt",
                                        
            Self::BrwsBack =>           "Browse Back",
            Self::BrwsForward =>        "Browse Fow",
            Self::BrwsRefresh =>        "Browse Refresh",
            Self::BrwsStop =>           "Browse Stop",
            Self::BrwsSearch =>         "Browse Search",
            Self::BrwsFavorite =>       "Browse Favorite",
            Self::BrwsHome =>           "Browse Home",
            Self::VlmMute =>            "Volume Mute",
            Self::VlmDown =>            "Volume Down",
            Self::VlmUp =>              "Volume Up",
            Self::MediaNext =>          "Media Next",
            Self::MediaPrev =>          "Media Previous",
            Self::MediaStop =>          "Media Stop",
            Self::MediaPlay =>          "Media Play",
            Self::LaunchMail =>         "Launch Mail",
            Self::LaunchMedia =>        "Launch Media",
            Self::LaunchApp1 =>         "Launch App 1",
            Self::LaunchApp2 =>         "Launch App 2",
            Self::AppsKey =>            "App Key",
            Self::PrintScreen =>        "Print Screen",
            Self::CtrlBreak =>          "Control Break",
                                        
            Self::Pause =>              "Pause",
            Self::Break =>              "Break",
            Self::Help =>               "Help",
            Self::Sleep =>              "Sleep",
            Self::Menu =>               "Menu",
            Self::Scroll =>             "Scroll",
            Self::Capital =>            "Capital",
            Self::Accent =>             "Accent",
            Self::Tilde =>              "~", // ~
            Self::Exclamation =>        "!",
            Self::At =>                 "@",
            Self::Hash =>               "#",
            Self::DollarSign =>         "$",
            Self::Percent =>            "%",
            Self::And =>                "&",
            Self::Astrix =>             "*",
            Self::OpenParentheses =>    "(",
            Self::CloseParentheses =>   ")",
            Self::Underscore =>         "_",
            Self::Minus =>              "-",
            Self::Plus =>               "+",
            Self::Equal =>              "=",
            Self::OpenBrace =>          "{",
            Self::CloseBrace =>         "}",
            Self::OpenBracket =>        "[",
            Self::CloseBracket =>       "]",
            Self::VerticalPipe =>       "|",
            Self::Backslash =>          "/",
            Self::Colon =>              ":",
            Self::Semicolon =>          ";",
            Self::QuotationMarks =>     "'",
            Self::Apostrophe =>         "Apostrophe",
            Self::Comma =>              ",",
            Self::Less =>               "<",
            Self::Greater =>            ">",
            Self::Period =>             ".",
            Self::Slash =>              "Slash",
            Self::QuestionMark =>       "?",
            Self::Grave =>              "'",
            Self::Unknown =>            "Unknown"
     

        };

        write!(f,"{}",k)


    }

}