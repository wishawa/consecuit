/** Internal use. Sealed.
 */
#[sealed::sealed]
pub trait StoresList: Sized + Default + 'static {
    type Head: Sized;
    type Tail: Sized + StoresList;
    fn create() -> Self;
}

/** Internal use.
 */
#[derive(Default)]
pub struct StoreCons<H: Default, T: Default>(pub(crate) H, pub(crate) T);

/** Internal use.
 */
#[derive(Default)]
pub struct StoreConsEnd;

#[sealed::sealed]
impl<H: Default + 'static, T: Default + 'static> StoresList for StoreCons<H, T>
where
    T: StoresList,
{
    type Head = H;
    type Tail = T;
    fn create() -> Self {
        Self(H::default(), T::create())
    }
}

#[sealed::sealed]
impl StoresList for StoreConsEnd {
    type Head = Self;
    type Tail = Self;
    fn create() -> Self {
        Self
    }
}
