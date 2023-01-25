// TODO: add comment

pub mod types;


use types::CEvent;
use types::input::*;
use types::window::WindowEvent;

use winit::event::{
    KeyboardInput,
    VirtualKeyCode,
    WindowEvent as WinitWindowEvent,
    ElementState,
    TouchPhase,
    MouseScrollDelta,
    Ime,
    ModifiersState,
    MouseButton as WinitMouseButton,
    Event as WEvent

};

use winit::event_loop::ControlFlow;

//
//
// ------------------------------------------------------------------------------------------------
// Event System
//
pub(crate) struct EventSystem {

    queue:                  EventQueue,
    modifier_state:         ModifierStateKeeper,
    call_to_close_app:      bool,

}
//
impl EventSystem {

    pub(crate) fn run_return(&mut self, event:WEvent<()>,mut ctrl_flow: ControlFlow) {

        ctrl_flow.set_wait();

        let e = match event {

            WEvent::WindowEvent{ event, .. } => convert_window_event(event),
            WEvent::Suspended => CEvent::Suspended,
            WEvent::Resumed => CEvent::Resumed,
            // TODO: take care of this event
            // WEvent::RedrawRequested() => ,
            WEvent::RedrawEventsCleared => CEvent::RedrawClear,
            WEvent::LoopDestroyed => CEvent::Ignored,

            _ =>  CEvent::Unknown

        };

        match e {
            CEvent::Ignored => {},
            _ => self.queue.add_event(e)
        }


    }
    //
    fn check_last_event(&mut self) {

        match self.queue.get_last_event() {

            CEvent::Window(event) => {
                match event {

                    WindowEvent::Close => self.call_to_close_app = true,


                    _ => {}

                }

            },

            _ => {}


        }

    }




}
//
//
// ------------------------------------------------------------------------------------------------
// Event queue Vec
//
pub(crate) struct EventQueue {

    queue:      Vec<CEvent>,

}
//
impl EventQueue {

    pub(crate) fn init(maxsize:Option<usize>) -> Self {

        match maxsize {

            Some(max) => Self { queue: Vec::with_capacity(max) },
            None => Self { queue: Vec::with_capacity(10000) }

        }

    }
    //
    pub(crate) fn add_event(&mut self,event:CEvent) {

        if self.queue.len() >= self.queue.capacity() {

            // Remove half of the content of the vec

            let half_capacity = self.queue.capacity() / 2;

            self.queue.drain(0 .. half_capacity);


        }

        self.queue.push(event)

    }
    //
    pub(crate) fn get_last_event(&self) -> &CEvent {

        match self.queue.last() {

            Some(cevent) => cevent,
            None =>     &CEvent::Any

        }

    }

}
//
//
// ------------------------------------------------------------------------------------------------
// Converter function
//
//
//
fn convert_window_event(wevent:WinitWindowEvent) -> CEvent {

    let win_event = match wevent {

        WinitWindowEvent::CloseRequested => WindowEvent::Close,
        WinitWindowEvent::Resized(size) => WindowEvent::Resize(size.width,size.height),
        WinitWindowEvent::Occluded(has_been) => {
            if has_been {
                WindowEvent::FullHidden
            } else {
                WindowEvent::PartiallyHidden
            }
        },
        WinitWindowEvent::Moved(pos) => WindowEvent::PositionChange(pos.x,pos.y),
        WinitWindowEvent::Destroyed => WindowEvent::Delete,
        WinitWindowEvent::DroppedFile(path) => WindowEvent::DropFile(path),
        WinitWindowEvent::HoveredFile(path) => WindowEvent::FileOver(path),
        WinitWindowEvent::HoveredFileCancelled => WindowEvent::FileEscaped,
        WinitWindowEvent::Focused(is_focused) => {

            if is_focused { WindowEvent::FocusGain }
            else { WindowEvent::FocusLost }

        },
        WinitWindowEvent::KeyboardInput { device_id,input,is_synthetic} =>
            return keyboard_event_converter(input),

        WinitWindowEvent::CursorLeft { .. } => return CEvent::Mouse(MouseEvent::MouseEscape),
        WinitWindowEvent::CursorEntered { .. } => return CEvent::Mouse(MouseEvent::MouseEntered),
        WinitWindowEvent::ReceivedCharacter(c) => return CEvent::ReceivedChar(c),
        WinitWindowEvent::Ime(ev) =>
            return match ev {
                Ime::Enabled => CEvent::TextInputModeEnabled,
                Ime::Disabled => CEvent::TextInputModeDisabled,
                Ime::Preedit(content, pos) => {
                    if content != "" {
                        return CEvent::TextInput(TextInputEvent::new(content, pos))
                    }

                    CEvent::Ignored
                },
                Ime::Commit(ctn) => CEvent::TextInput(TextInputEvent::new(ctn, None))
            },
        WinitWindowEvent::ModifiersChanged(mstate) => return modifier_event_converter(mstate),
        WinitWindowEvent::CursorMoved { position, ..} => {

            let mouse_ev = MouseEvent::MouseMovement(MouseMovedEvent::new(position.x,position.y));

            return CEvent::Mouse(mouse_ev)

        },
        WinitWindowEvent::MouseWheel { delta, phase, ..} => {

            let p = match phase {

                TouchPhase::Started => PhaseWheelScroll::Started,
                TouchPhase::Moved => PhaseWheelScroll::Moved,
                TouchPhase::Ended => PhaseWheelScroll::Stopped,
                TouchPhase::Cancelled => PhaseWheelScroll::Canceled

            };

            let (col,row) = match delta {
                MouseScrollDelta::LineDelta(col,row) => (col,row),
                // TODO: add the other possible closure
                _  => return CEvent::Ignored
            };

            let mouse_ev = MouseWheelRollEvent::new(p,col,row);

            return CEvent::Mouse(MouseEvent::MouseWheelRoll(mouse_ev))


        },
        WinitWindowEvent::MouseInput { state, button, .. } => {

            let action = match state {

                ElementState::Pressed => StateAction::Press,
                ElementState::Released => StateAction::Release

            };

            let button = match button {

                WinitMouseButton::Right => MouseButton::Right,
                WinitMouseButton::Left => MouseButton::Left,
                WinitMouseButton::Middle => MouseButton::Wheel,
                _ => MouseButton::Unknown

            };

            let mouse_ev = MouseButtonEvent::new(button,action);

            return CEvent::Mouse(MouseEvent::MouseButton(mouse_ev))


        }



        _ => return CEvent::Unknown


    };

    CEvent::Window(win_event)

}
//
fn modifier_event_converter(mstate:ModifiersState) -> CEvent {

    let mevent = if mstate.intersects(ModifiersState::CTRL) {

        ModifierChangeEvent::new(CONTROL)

    } else if mstate.intersects(ModifiersState::ALT) {

        ModifierChangeEvent::new(ALT)

    } else if mstate.intersects(ModifiersState::SHIFT) {

        ModifierChangeEvent::new(SHIFT)

    } else if mstate.intersects(ModifiersState::LOGO) {

        ModifierChangeEvent::new(SUPER)

    } else if mstate.intersects(ModifiersState::CTRL | ModifiersState::ALT ) {

        ModifierChangeEvent::new(CONTROL_ALT)

    } else if mstate.intersects(ModifiersState::CTRL | ModifiersState::SHIFT) {

        ModifierChangeEvent::new(CONTROL_SHIFT)

    } else if
    mstate.intersects(ModifiersState::CTRL | ModifiersState::ALT | ModifiersState::SHIFT ) {

        ModifierChangeEvent::new(CONTROL_ALT_SHIFT)

    } else {

        ModifierChangeEvent::new(NULL_MOD)

    };

    CEvent::ModifierStateChange(mevent)


}
//
fn keyboard_event_converter(key_event:KeyboardInput) -> CEvent {


    let kevent = match key_event.virtual_keycode {

        Some(vcode) => {

            let key = match vcode {

                VirtualKeyCode::A =>                Key::A,
                VirtualKeyCode::B =>                Key::B,
                VirtualKeyCode::C =>                Key::C,
                VirtualKeyCode::D =>                Key::D,
                VirtualKeyCode::E =>                Key::E,
                VirtualKeyCode::F =>                Key::F,
                VirtualKeyCode::G =>                Key::G,
                VirtualKeyCode::H =>                Key::H,
                VirtualKeyCode::I =>                Key::I,
                VirtualKeyCode::J =>                Key::J,
                VirtualKeyCode::K =>                Key::K,
                VirtualKeyCode::L =>                Key::L,
                VirtualKeyCode::M =>                Key::M,
                VirtualKeyCode::N =>                Key::N,
                VirtualKeyCode::O =>                Key::O,
                VirtualKeyCode::P =>                Key::P,
                VirtualKeyCode::Q =>                Key::Q,
                VirtualKeyCode::R =>                Key::R,
                VirtualKeyCode::S =>                Key::S,
                VirtualKeyCode::T =>                Key::T,
                VirtualKeyCode::U =>                Key::U,
                VirtualKeyCode::V =>                Key::V,
                VirtualKeyCode::W =>                Key::W,
                VirtualKeyCode::X =>                Key::X,
                VirtualKeyCode::Y =>                Key::Y,
                VirtualKeyCode::Z =>                Key::Z,
                // function key
                VirtualKeyCode::F1 =>               Key::F1,
                VirtualKeyCode::F2 =>               Key::F2,
                VirtualKeyCode::F3 =>               Key::F3,
                VirtualKeyCode::F4 =>               Key::F4,
                VirtualKeyCode::F5 =>               Key::F5,
                VirtualKeyCode::F6 =>               Key::F6,
                VirtualKeyCode::F7 =>               Key::F7,
                VirtualKeyCode::F8 =>               Key::F8,
                VirtualKeyCode::F9 =>               Key::F9,
                VirtualKeyCode::F10 =>              Key::F10,
                VirtualKeyCode::F11 =>              Key::F11,
                VirtualKeyCode::F12 =>              Key::F12,
                // Modifier
                VirtualKeyCode::LWin     =>         Key::LSuper,
                VirtualKeyCode::RWin     =>         Key::RSuper,
                VirtualKeyCode::LAlt     =>         Key::LAlt,
                VirtualKeyCode::RAlt     =>         Key::RAlt,
                VirtualKeyCode::LControl =>         Key::LCtrl,
                VirtualKeyCode::RControl =>         Key::RCtrl,
                VirtualKeyCode::LShift   =>         Key::LShift,
                VirtualKeyCode::RShift   =>         Key::RShift,
                // multimedia

                // TODO: finish

                // Number
                VirtualKeyCode::Key0 =>             Key::N0,
                VirtualKeyCode::Key1 =>             Key::N1,
                VirtualKeyCode::Key2 =>             Key::N2,
                VirtualKeyCode::Key3 =>             Key::N3,
                VirtualKeyCode::Key4 =>             Key::N4,
                VirtualKeyCode::Key5 =>             Key::N5,
                VirtualKeyCode::Key6 =>             Key::N6,
                VirtualKeyCode::Key7 =>             Key::N7,
                VirtualKeyCode::Key8 =>             Key::N8,
                VirtualKeyCode::Key9 =>             Key::N9,
                // numpad
                VirtualKeyCode::Numlock =>          Key::NpLock,
                VirtualKeyCode::NumpadDivide =>     Key::NpDiv,
                VirtualKeyCode::NumpadMultiply =>   Key::NpMult,
                VirtualKeyCode::NumpadAdd =>        Key::NpAdd,
                VirtualKeyCode::NumpadSubtract =>   Key::NpSub,
                VirtualKeyCode::NumpadEnter =>      Key::NpEnter,
                VirtualKeyCode::NumpadDecimal =>    Key::NpDot,
                VirtualKeyCode::Numpad0 =>          Key::Np0,
                VirtualKeyCode::Numpad1 =>          Key::Np1,
                VirtualKeyCode::Numpad2 =>          Key::Np2,
                VirtualKeyCode::Numpad3 =>          Key::Np3,
                VirtualKeyCode::Numpad4 =>          Key::Np4,
                VirtualKeyCode::Numpad5 =>          Key::Np5,
                VirtualKeyCode::Numpad6 =>          Key::Np6,
                VirtualKeyCode::Numpad7 =>          Key::Np7,
                VirtualKeyCode::Numpad8 =>          Key::Np8,
                VirtualKeyCode::Numpad9 =>          Key::Np9,
                // other
                VirtualKeyCode::Insert =>           Key::Insert,
                VirtualKeyCode::End =>              Key::End,
                VirtualKeyCode::PageUp =>           Key::PgUp,
                VirtualKeyCode::PageDown =>         Key::PgDn,
                VirtualKeyCode::Home =>             Key::Home,
                VirtualKeyCode::Up =>               Key::Up,
                VirtualKeyCode::Down =>             Key::Down,
                VirtualKeyCode::Left =>             Key::Left,
                VirtualKeyCode::Right =>            Key::Right,
                VirtualKeyCode::Delete =>           Key::Delete,
                VirtualKeyCode::Pause =>            Key::Pause,
                VirtualKeyCode::Sleep =>            Key::Sleep,
                VirtualKeyCode::Escape =>           Key::Escape,
                VirtualKeyCode::Scroll =>           Key::Scroll,
                VirtualKeyCode::Return =>           Key::Return,
                VirtualKeyCode::Space =>            Key::Space,
                // Symbol
                VirtualKeyCode::At =>               Key::At,
                VirtualKeyCode::Equals =>           Key::Equal,
                VirtualKeyCode::Minus =>            Key::Minus,
                VirtualKeyCode::Backslash =>        Key::Backslash,
                VirtualKeyCode::LBracket =>         Key::OpenBracket,
                VirtualKeyCode::RBracket =>         Key::CloseBracket,
                VirtualKeyCode::Colon =>            Key::Colon,
                VirtualKeyCode::Capital =>          Key::Capital,
                VirtualKeyCode::Semicolon =>        Key::Semicolon,
                VirtualKeyCode::Comma =>            Key::Comma,
                VirtualKeyCode::Grave =>            Key::Grave,
                VirtualKeyCode::Period =>           Key::Period,

                _ =>                                Key::Unknown

            };

            let action = match key_event.state {

                ElementState::Pressed =>  StateAction::Press,
                ElementState::Released => StateAction::Release,

            };

            KeyboardEvent::new(key,action)

        },
        None => {

            let action = match key_event.state {

                ElementState::Pressed =>  StateAction::Press,
                ElementState::Released => StateAction::Release,

            };

            KeyboardEvent::new(Key::Unknown,action)


        }

    };

    CEvent::Keyboard(kevent)

}














