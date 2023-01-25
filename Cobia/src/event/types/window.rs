
// TODO: add comment

use std::path::PathBuf;

pub enum WindowEvent {

    PositionChange(i32,i32),
    Resize(u32,u32),
    Close,
    ResolutionChange,
    FullHidden,
    PartiallyHidden,
    Delete,
    DropFile(PathBuf),
    // File is over the window
    FileOver(PathBuf),
    // File exit the screen after been over the window
    FileEscaped,
    FocusLost,
    FocusGain,




}