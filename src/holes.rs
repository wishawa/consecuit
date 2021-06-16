use web_sys::Node;

pub trait HolesList: Sized + Clone {
    type Tail: Sized + HolesList;
	fn append(self, node: Node) -> HoleCons<Self>;
	fn split_head(self) -> (Node, Self::Tail);
}

#[derive(Clone)]
pub struct HoleCons<T>(pub(crate) Node, pub(crate) T);

#[derive(Clone)]
pub struct HoleConsEnd;

impl<T: HolesList + Sized> HolesList for HoleCons<T> {
	type Tail = T;
	fn append(self, node: Node) -> HoleCons<Self> {
		HoleCons (self.0, self.1.append(node))
	}
	fn split_head(self) -> (Node, Self::Tail) {
		(self.0, self.1)
	}
}

impl HolesList for HoleConsEnd {
    type Tail = Self;
	fn append(self, node: Node) -> HoleCons<Self> {
		HoleCons (node, self)
	}
	fn split_head(self) -> (Node, Self::Tail) {
		panic!("child added to component with no remaining holes")
	}
}

pub trait HolesListNotEmpty {

}

impl<T: HolesList> HolesListNotEmpty for HoleCons<T> {}