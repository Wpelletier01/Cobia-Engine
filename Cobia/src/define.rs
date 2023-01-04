
use ash::vk::make_api_version;

#[cfg(debug_assertions)]
pub(crate) const CRELEASE:u8 = 0;

#[cfg(not(debug_assertions))]
pub(crate) const CRELEASE:u8 = 1;


pub const FHOUR_AS_SECONDS: f32 =   3600.0;
pub const FMIN_AS_SECONDS:  f32 =   60.0;

pub const VLK_APP_VERSION:      u32 = 0;
pub const VLK_ENGINE_VERSION:   u32 = 0;
pub const VLK_API_VERSION:      u32 = make_api_version(0, 1, 0, 0);

//
//
// ------------------------------------------------------------------------------------------------
// 
