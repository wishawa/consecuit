use web_sys::Element;

use crate::stores::StoresList;

use super::ComponentStores;

pub trait MaybeHoleNode: 'static + Clone {}
#[derive(Clone)]
pub struct NoHoleNode;
impl MaybeHoleNode for NoHoleNode {}
#[derive(Clone)]
pub struct YesHoleNode(pub Element);
impl MaybeHoleNode for YesHoleNode {}

impl<Stores, EntireStores> ComponentStores<Stores, EntireStores, YesHoleNode, NoHoleNode>
where
    Stores: StoresList,
    EntireStores: StoresList,
{
    pub fn hole_here(self) -> ComponentStores<Stores, EntireStores, NoHoleNode, YesHoleNode> {
        let ComponentStores {
            hook_stores,
            parent_node,
            last_node,
            ret_node,
        } = self;
        ComponentStores {
            hook_stores,
            parent_node,
            last_node: ret_node,
            ret_node: last_node,
        }
    }
}
