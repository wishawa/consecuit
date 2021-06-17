use web_sys::Element;

use crate::stores::{StoreConsEnd, StoresList};

use super::{
    hole::{NoHoleNode, YesHoleNode},
    ComponentStores,
};

impl<EntireStores> ComponentStores<StoreConsEnd, EntireStores, NoHoleNode, NoHoleNode>
where
    EntireStores: StoresList,
{
    pub(crate) fn bare_container_node(
        self,
        node: Element,
    ) -> ComponentStores<StoreConsEnd, EntireStores, NoHoleNode, YesHoleNode> {
        let ComponentStores {
            hook_stores,
            parent_node,
            ..
        } = self;
        ComponentStores {
            hook_stores,
            parent_node,
            last_node: NoHoleNode,
            ret_node: YesHoleNode(node),
        }
    }
    pub(crate) fn bare_leaf_node(
        self,
    ) -> ComponentStores<StoreConsEnd, EntireStores, NoHoleNode, NoHoleNode> {
        let ComponentStores {
            hook_stores,
            parent_node,
            ..
        } = self;
        ComponentStores {
            hook_stores,
            parent_node,
            last_node: NoHoleNode,
            ret_node: NoHoleNode,
        }
    }
}
