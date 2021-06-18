use web_sys::Element;

use crate::stores::{StoreConsEnd, StoresList};

use super::{
    hole::{NoHoleNode, YesHoleNode},
    ComponentConstruction,
};

impl<EntireStores> ComponentConstruction<StoreConsEnd, EntireStores, NoHoleNode, NoHoleNode>
where
    EntireStores: StoresList,
{
    pub(crate) fn bare_container_node(
        self,
        node: Element,
    ) -> ComponentConstruction<StoreConsEnd, EntireStores, NoHoleNode, YesHoleNode> {
        let ComponentConstruction {
            hook_stores,
            parent_node,
            ..
        } = self;
        ComponentConstruction {
            hook_stores,
            parent_node,
            last_node: NoHoleNode,
            ret_node: YesHoleNode(node),
        }
    }
    pub(crate) fn bare_leaf_node(
        self,
    ) -> ComponentConstruction<StoreConsEnd, EntireStores, NoHoleNode, NoHoleNode> {
        let ComponentConstruction {
            hook_stores,
            parent_node,
            ..
        } = self;
        ComponentConstruction {
            hook_stores,
            parent_node,
            last_node: NoHoleNode,
            ret_node: NoHoleNode,
        }
    }
}
