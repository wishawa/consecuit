use std::cell::RefCell;

use crate::stores::{StoreCons, StoreConsEnd, StoresList};

use super::{
    component::{ComponentBuilder, ComponentConstruction},
    hole::{MaybeHoleNode, NoHoleNode, YesHoleNode},
    hook::HookConstruction,
    subtree::Subtree,
};

#[sealed::sealed]
pub trait ComponentReturn: 'static {
    type StoresList: StoresList;
    type HoleNode: MaybeHoleNode;
    fn get_node(self) -> Self::HoleNode;
}
#[sealed::sealed]
pub trait ContainerReturn: ComponentReturn<HoleNode = YesHoleNode> {}

#[sealed::sealed]
impl<Stores: StoresList, LastNode: MaybeHoleNode, HoleNode: MaybeHoleNode> ComponentReturn
    for ComponentConstruction<StoreConsEnd, Stores, LastNode, HoleNode>
{
    type StoresList = Stores;
    type HoleNode = HoleNode;
    fn get_node(self) -> Self::HoleNode {
        self.ret_node
    }
}
#[sealed::sealed]
impl<T: ComponentReturn<HoleNode = YesHoleNode>> ContainerReturn for T {}

#[sealed::sealed]
pub trait ComponentProps: PartialEq + Clone + 'static {}
#[sealed::sealed]
impl<T: PartialEq + Clone + 'static> ComponentProps for T {}

pub type ComponentFunc<Ret, Props> = fn(ComponentBuilder, Props) -> Ret;

pub type DynComponentReturn<Props> = ComponentConstruction<
    StoreConsEnd,
    StoreCons<RefCell<Option<Box<dyn Subtree<Props = Props>>>>, StoreConsEnd>,
    NoHoleNode,
    NoHoleNode,
>;

#[sealed::sealed]
pub trait HookReturn<Value> {
    type StoresList: StoresList;
    fn get_val(self) -> Value;
}

#[sealed::sealed]
impl<UsedStores, Value> HookReturn<Value> for (HookConstruction<StoreConsEnd, UsedStores>, Value)
where
    UsedStores: StoresList,
{
    type StoresList = UsedStores;
    fn get_val(self) -> Value {
        self.1
    }
}
