use crate::{stores::StoresList, ComponentBuilder};

use super::hole::{MaybeHoleNode, YesHoleNode};

pub trait ComponentProps: PartialEq + Clone + 'static {}
impl<T: PartialEq + Clone + 'static> ComponentProps for T {}

pub type ComponentFunc<Props, Ret> = fn(ComponentBuilder, Props) -> Ret;

pub trait ComponentReturn: 'static {
    type StoresList: StoresList;
    type HoleNode: MaybeHoleNode;
    fn get_node(self) -> Self::HoleNode;
}

pub trait ContainerReturn: ComponentReturn<HoleNode = YesHoleNode> {}
impl<T: ComponentReturn<HoleNode = YesHoleNode>> ContainerReturn for T {}
