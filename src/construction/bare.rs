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
    /** Mark this component as a base container component.

    This is for creating your own base component.

    If you stick with the ones provided by the `consecuit_html` crate, you won't need this.

    Look at `consecuit_html`'s source code for example on how to create base components.

    Note that when creating base components,
    you are stepping outside the "functional" model, and some rules apply:

    * Nodes should be created on initial render.
    * You can create nodes in later renders, but they must be descendant of the nodes created on initial render.
    * The node you pass to `bare_container_node` should be the same node in all render.

    */
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

    /** Mark this component as a base leaf (childless) component.

    This is for creating your own base component.

    If you stick with the ones provided by the `consecuit_html` crate, you won't need this.
    Look at `consecuit_html`'s source code for example on how to create base components.

    Note that when creating base components,
    you are stepping outside the "functional" model, and some rules apply:

    * Nodes should be created on initial render.
    * You can create nodes in later renders, but they must be descendant of the nodes created on initial render.

    */
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
