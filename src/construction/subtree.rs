use std::{borrow::Borrow, marker::PhantomData};
use web_sys::{window, Node};

use crate::{
    locking::SharedPart,
    prelude::HookBuilder,
    stores::{StoreCons, StoresList},
};

use super::{
    component::{ComponentConstruction, ComponentStore, ComponentStoreInstance},
    hole::NoHoleNode,
    hook::HookConstruction,
    types::{ComponentFunc, ComponentProps, ComponentReturn},
};

type TreeStores<Ret, Props> =
    StoreCons<ComponentStoreInstance<Ret, Props>, <Ret as ComponentReturn>::StoresList>;

/** A subtree is a part of the app that is mounted and unmounted together.

When [mounting the app][crate::construction::mount::mount_app], Consecuit creates a subtree for it.

[`opt_comp`][crate::construction::subtrees::opt_comp] creates a subtree for its component.
[`vec_comps`][crate::construction::subtrees::vec_comps] creates a subtree for each of its components.
 */
pub struct SubtreeInstance<Ret, Props>
where
    Ret: ComponentReturn,
    Props: ComponentProps,
{
    stores: SharedPart<TreeStores<Ret, Props>>,
    parent: Node,
    nodes: Vec<Node>,
    func: ComponentFunc<Ret, Props>,
}

impl<Ret, Props> Drop for SubtreeInstance<Ret, Props>
where
    Ret: ComponentReturn,
    Props: ComponentProps,
{
    fn drop(&mut self) {
        self.stores.unmount_tree();
        self.nodes.iter().for_each(|node| {
            self.parent.remove_child(node).unwrap();
        });
    }
}

/** Internal use. Sealed.

*/
#[sealed::sealed]
pub trait Subtree {
    type Props;
    fn re_render(&self, props: Self::Props);
}

#[sealed::sealed]
impl<Ret, Props> Subtree for SubtreeInstance<Ret, Props>
where
    Ret: ComponentReturn,
    Props: ComponentProps,
{
    type Props = Props;
    fn re_render(&self, props: Self::Props) {
        struct DummyTreeRoot;
        impl ComponentStore for DummyTreeRoot {
            fn dyn_render(&self, _hb: HookBuilder) {
                unreachable!("this dummy is never directly called")
            }

            fn map_stores(&self) -> &dyn std::any::Any {
                unreachable!("this dummy is never directly called")
            }
        }
        let dummy_root = SharedPart::new(DummyTreeRoot).map(|p| p as &dyn ComponentStore);

        let stores = self.stores.borrow().clone();

        type StillFullNodeComponentStore<T> = ComponentConstruction<T, T, NoHoleNode, NoHoleNode>;
        let component_store: StillFullNodeComponentStore<_> = ComponentConstruction {
            hook_stores: HookConstruction {
                current: stores,
                entire: PhantomData,
                current_component: dummy_root.clone(),
            },
            parent_node: self.parent.clone(),
            last_node: NoHoleNode,
            ret_node: NoHoleNode,
        };
        component_store.comp(self.func, props);
    }
}

pub(crate) fn mount_subtree<Ret, Props>(
    func: ComponentFunc<Ret, Props>,
    props: Props,
    container: Node,
) -> SubtreeInstance<Ret, Props>
where
    Ret: ComponentReturn,
    Props: ComponentProps,
{
    let stores: TreeStores<Ret, Props> = StoresList::create();
    let stores = SharedPart::new(stores);
    let document = window().unwrap().document().unwrap();
    let fragment = document.create_document_fragment();
    let parent = fragment.clone().into();
    let mut subtree = SubtreeInstance {
        stores,
        parent,
        nodes: Vec::new(),
        func,
    };
    subtree.re_render(props);
    let child_nodes = fragment.child_nodes();
    let length = child_nodes.length();
    let nodes = (0..length).map(|i| child_nodes.item(i).unwrap()).collect();
    container.append_child(&fragment).unwrap();
    subtree.nodes = nodes;
    subtree.parent = container;
    subtree
}
