pub mod authentication;
pub mod note;
pub(crate) trait Sealed {}
/// Gets anything as converted to the latest used version of that object
#[allow(private_bounds)]
pub trait Latest<V1: APIV1, T>: Sealed + AsAPIV1<V1> {
    fn get_latest(self) -> T;
}
#[allow(private_bounds)]
pub trait AsAPIV1<T: APIV1>: Sealed {
    fn to_v1(self) -> Result<T, crate::Error>;
}
pub(crate) trait APIV1: Sealed {}
