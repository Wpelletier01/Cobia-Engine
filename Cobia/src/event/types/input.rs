// TODO: add comment

use super::CEvent;

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
pub const CONTROL_ALT_SHIFT:        u8 = 7;
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
//
// ------------------------------------------------------------------------------------------------
// Mouse Event
//

pub enum MouseEvent {

    MouseButton(MouseButtonEvent),
    MouseMovement(MouseMovedEvent),
    MouseEscape,
    MouseEntered,
    MousePassed, // mouse Passed on the window
    MouseWheelRoll(MouseWheelRollEvent)

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

    content: String,
    position:  Option<(usize,usize)>

}
//
impl TextInputEvent {

    pub fn new(content:String,position:Option<(usize,usize)>) -> Self { Self { content, position } }

}
//
//
// ------------------------------------------------------------------------------------------------
// Keyboard
//
pub struct KeyboardEvent {

    key:        Key,
    action:     StateAction,

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