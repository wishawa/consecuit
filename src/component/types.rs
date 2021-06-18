use std::cell::RefCell;

use crate::{
    stores::{StoreCons, StoreConsEnd, StoresList},
    ComponentBuilder,
};

use super::{
    hole::{MaybeHoleNode, NoHoleNode, YesHoleNode},
    subtree::Subtree,
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

pub type DynComponentReturn<P> = ComponentConstruction<
    StoreConsEnd,
    StoreCons<RefCell<Option<Box<dyn Subtree<Props = P>>>>, StoreConsEnd>,
    NoHoleNode,
    NoHoleNode,
>;

pub trait ComponentProps: PartialEq + Clone + 'static {}
impl<T: PartialEq + Clone + 'static> ComponentProps for T {}

pub type ComponentFunc<Ret, Props> = fn(ComponentBuilder, Props) -> Ret;
