
// TODO: add comment

use std::path::PathBuf;
use std::fmt::{Display, Formatter, Pointer, Result};

pub enum WindowEvent {

    PositionChange(i32,i32),
    Resize(u32,u32),
    Close,
    ResolutionChange(ResolutionChangeEvent),
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
//
impl Display for WindowEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {

        match self {

            Self::PositionChange(x,y) =>
                write!(f,"Position change at x: {} px y: {} px ",x,y),
            Self::Resize(w,h) =>
                write!(f,"Resize to width: {} px height: {} px",w,h),
            Self::Close =>
                write!(f,"Closing Window call"),
            Self::ResolutionChange(res) =>
                write!(f,"Resolution change with scale factor of {} Inner size width: {} height: \
                {}", res.scale_fact,res.new_inner_size.0,res.new_inner_size.1),
            Self::Delete =>
                write!(f,"Window will be destroy"),
            Self::DropFile(p) =>
                write!(f,"File '{}' have been dropped on the window", p.to_str().unwrap()),
            Self::FileOver(p) =>
                write!(f,"File '{}' is over the window", p.to_str().unwrap()),
            Self::FileEscaped =>
                write!(f,"The last file has escaped the window"),
            Self::FocusGain =>
                write!(f,"Gain focus of the window"),
            Self::FocusLost =>
                write!(f,"Loose focus of the window"),
            Self::FullHidden =>
                write!(f,"Window Fully hidden"),
            Self::PartiallyHidden =>
                write!(f,"Window Partially hidden")



        }

    }

}
//
//
pub struct ResolutionChangeEvent {

    scale_fact: f64,
    new_inner_size: (u32,u32)

}
//
impl ResolutionChangeEvent {

    pub fn new(scale_fact: f64,new_inner_size:(u32,u32)) -> Self {
        Self { scale_fact, new_inner_size }
    }

}