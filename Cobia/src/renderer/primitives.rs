





// float vertex  
pub type FVec2 = [f32; 2];
pub type FVec3 = [f32; 3];
// double vertex
pub type DVec2 = [f32;2];
pub type DVec3 = [f32;3];
// indices
pub type Indice = u16;


pub struct FVertex {

    pub position:   FVec3,
    pub normal:     FVec3,
    pub tex_coord:  FVec2,

}

pub struct DVertex {

    pub position:   DVec3,
    pub normal:     DVec3,
    pub tex_coord:  DVec2,

}