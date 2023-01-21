
pub mod datatypes;

pub(crate) struct EventQueue<T> where T: Event {

    queue: Vec<T>

}







pub trait Event {

    fn get_type(&self) ->


}






