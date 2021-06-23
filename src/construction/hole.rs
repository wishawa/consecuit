use web_sys::Element;

use crate::stores::StoresList;

use super::component::ComponentConstruction;

/// Marker trait used internally. Sealed.
#[sealed::sealed]
pub trait MaybeHoleNode: 'static + Clone {}
#[derive(Clone)]

/// Internal use.
pub struct NoHoleNode;
#[sealed::sealed]
impl MaybeHoleNode for NoHoleNode {}
#[derive(Clone)]

/// Internal use.
pub struct YesHoleNode(pub(crate) Element);
#[sealed::sealed]
impl MaybeHoleNode for YesHoleNode {}

impl<Stores, EntireStores> ComponentConstruction<Stores, EntireStores, YesHoleNode, NoHoleNode>
where
    Stores: StoresList,
    EntireStores: StoresList,
{
    /// Mark the lastest comp as the Hole of this component.
    ///
    /// Calling `.child(...)` on this component will put the children there.
    ///
    /// A component that returns `impl ContainerReturn` must have exactly one `.hole_here()`.
    ///
    /// The [`crate::reia_tree`] macro equivalent to this is the `HOLE` attribute. Like
    /// `<div HOLE />`
    pub fn hole_here(self) -> ComponentConstruction<Stores, EntireStores, NoHoleNode, YesHoleNode> {
        let ComponentConstruction {
            hook_stores,
            parent_node,
            last_node,
            ..
        } = self;
        ComponentConstruction {
            hook_stores,
            parent_node,
            last_node: NoHoleNode,
            ret_node: last_node,
        }
    }
}
