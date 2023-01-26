use std::string::ToString;

#[cfg(debug_assertions)]
pub(crate) const CRELEASE:u8 = 0;

#[cfg(not(debug_assertions))]
pub(crate) const CRELEASE:u8 = 1;


pub const FHOUR_AS_SECONDS: f32 =   3600.0;
pub const FMIN_AS_SECONDS:  f32 =   60.0;

pub const ENGINE_VERSION:  (u32,u32,u32) = (1,0,0);



//
//
// ------------------------------------------------------------------------------------------------
// 
