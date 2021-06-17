use crate::ComponentBuilder;

pub trait ComponentProps: PartialEq + Clone + 'static {}
impl<T: PartialEq + Clone + 'static> ComponentProps for T {}

pub type ComponentFunc<Props, Ret> = fn(ComponentBuilder, Props) -> Ret;
