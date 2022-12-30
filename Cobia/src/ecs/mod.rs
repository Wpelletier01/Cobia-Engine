#[allow(dead_code)]

enum CType {

    IMAGE 


}



trait Component {
    //
    /// get th id of the component 
    /// TODO: maybe it represents the index of the component in an array of components?
    fn get_id(&self) -> u32;
    //
    /// return the type of the component
    fn get_type(&self) -> CType;
    //
}
//
//
// ------------------------------------------------------------------------------------------------
// IMAGE
// 
//
pub trait ImageTrait<T> {
    // 
    /// returns the widht of the image
    fn get_width(&self) -> u16;
    //
    /// return the height of the image
    fn get_height(&self) -> u16;
    //
    // returns a flattened array of pixels
    fn get_data(&self) -> &[T];
    //

}
//
//
/// Lowest representation of an image in this engine
pub(crate) struct RgbImage<T> {

    id:         u32,
    width:      u16,
    height:     u16,
    data:       [T], // The generic represent the bitrate
    
}
//
impl<T> Component for RgbImage<T> { 

    fn get_id(&self) -> u32 { self.id }

    fn get_type(&self) -> CType { CType::IMAGE } 

}
//
impl<T> ImageTrait<T> for RgbImage<T> {
    
    fn get_height(&self) -> u16 { self.width }

    fn get_width(&self) -> u16 { self.height }

    fn get_data(&self) -> &[T] { &self.data }

}
//

