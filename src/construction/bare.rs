use web_sys::Node;

use crate::stores::{StoreConsEnd, StoresList};

use super::{
    component::ComponentConstruction,
    hole::{NoHoleNode, YesHoleNode},
};

impl<EntireStores> ComponentConstruction<StoreConsEnd, EntireStores, NoHoleNode, NoHoleNode>
where
    EntireStores: StoresList,
{
    /// Mark this component as a base container component.
    ///
    /// This is for creating your own base component.
    /// If you stick with the ones provided by the `consecuit_html` crate, you won't need this.
    ///
    /// If you want to use this, use `consecuit_html`'s source code as example.
    pub fn bare_container_node(
        self,
        node: Node,
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

    /// Mark this component as a base leaf (childless) component.
    ///
    /// This is for creating your own base component.
    /// If you stick with the ones provided by the `consecuit_html` crate, you won't need this.
    ///
    /// If you want to use this, use `consecuit_html`'s source code as example.
    pub fn bare_leaf_node(
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
