use web_sys::Element;

use crate::stores::StoresList;

use super::ComponentConstruction;

#[sealed::sealed]
pub trait MaybeHoleNode: 'static + Clone {}
#[derive(Clone)]
pub struct NoHoleNode;
#[sealed::sealed]
impl MaybeHoleNode for NoHoleNode {}
#[derive(Clone)]
pub struct YesHoleNode(pub Element);
#[sealed::sealed]
impl MaybeHoleNode for YesHoleNode {}

impl<Stores, EntireStores> ComponentConstruction<Stores, EntireStores, YesHoleNode, NoHoleNode>
where
    Stores: StoresList,
    EntireStores: StoresList,
{
    pub fn hole_here(
        self,
    ) -> ComponentConstruction<Stores, EntireStores, YesHoleNode, YesHoleNode> {
        let ComponentConstruction {
            hook_stores,
            parent_node,
            last_node,
            ..
        } = self;
        ComponentConstruction {
            hook_stores,
            parent_node,
            last_node: last_node.clone(),
            ret_node: last_node,
        }
    }
}
