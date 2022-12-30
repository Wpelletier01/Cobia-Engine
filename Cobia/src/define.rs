

#[cfg(debug_assertions)]
pub(crate) const CRELEASE:u8 = 0;

#[cfg(not(debug_assertions))]
pub(crate) const CRELEASE:u8 = 1;


pub const FHOUR_AS_SECONDS: f32 =   3600.0;
pub const FMIN_AS_SECONDS:  f32 =   60.0;


//
//
// ------------------------------------------------------------------------------------------------
// 
