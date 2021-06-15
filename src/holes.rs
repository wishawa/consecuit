use web_sys::Node;

pub trait HoleNodesList: 'static + Clone + Sized {
}

#[derive(Clone)]
pub struct HoleNodeCons<T: HoleNodesList> (pub(crate) Node, pub(crate) T);

#[derive(Clone)]
pub struct HoleNodeConsEnd ();

impl<T: HoleNodesList> HoleNodesList for HoleNodeCons<T> {
}

impl HoleNodesList for HoleNodeConsEnd {
}