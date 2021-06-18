use crate::{
    stores::{StoreConsEnd, StoresList},
    ComponentBuilder,
};

use super::{
    hole::{MaybeHoleNode, YesHoleNode},
    ComponentConstruction,
};

pub trait ComponentReturn: 'static {
    type StoresList: StoresList;
    type HoleNode: MaybeHoleNode;
    fn get_node(self) -> Self::HoleNode;
}

pub trait ContainerReturn: ComponentReturn<HoleNode = YesHoleNode> {}
impl<T: ComponentReturn<HoleNode = YesHoleNode>> ContainerReturn for T {}

impl<Stores: StoresList, LastNode: MaybeHoleNode, HoleNode: MaybeHoleNode> ComponentReturn
    for ComponentConstruction<StoreConsEnd, Stores, LastNode, HoleNode>
{
    type StoresList = Stores;
    type HoleNode = HoleNode;
    fn get_node(self) -> Self::HoleNode {
        self.ret_node
    }
}

pub trait ComponentProps: PartialEq + Clone + 'static {}
impl<T: PartialEq + Clone + 'static> ComponentProps for T {}

pub type ComponentFunc<Ret, Props> = fn(ComponentBuilder, Props) -> Ret;
