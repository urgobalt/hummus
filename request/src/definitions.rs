mod note;
pub use note::*;
#[allow(private_bounds)]
pub(crate) trait Sealed {}
/// Gets anything as converted to the latest used version of that object
pub trait Latest<T>: Sealed {
    fn get_latest(self) -> T;
}
