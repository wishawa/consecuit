use crate::{ComponentBuilder, ComponentReturn};

pub trait ComponentProps: PropsPartialEq + Clone + 'static {}

impl<T: PropsPartialEq + Clone + 'static> ComponentProps for T {}

pub trait PropsPartialEq {
	fn props_eq(&self, other: &Self) -> bool;
}

impl<T: PartialEq> PropsPartialEq for T {
	fn props_eq(&self, other: &Self) -> bool {
		self.eq(other)
	}
}
