
use bytemuck::{Pod,Zeroable};


// float vertex  
pub type FVec2 = [f32; 2];
pub type FVec3 = [f32; 3];
// double vertex
pub type DVec2 = [f32;2];
pub type DVec3 = [f32;3];
// indices
pub type Indice = u16;



#[repr(C)]
#[derive(Clone,Copy,Debug,Default,Zeroable,Pod)]
pub struct Vertex {
    pub position:  FVec2

}

