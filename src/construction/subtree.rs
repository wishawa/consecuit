use std::{borrow::Borrow, marker::PhantomData, mem::transmute};
use web_sys::{window, Node};

use crate::{
    locking::UnmountedLock,
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
    stores: Box<TreeStores<Ret, Props>>,
    lock: UnmountedLock,
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
        self.lock.unmount();
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
            fn render(&'static self) {
                unreachable!("this dummy is never directly called")
            }
        }
        static DUMMY_ROOT: DummyTreeRoot = DummyTreeRoot;

        let stores_borrow = unsafe {
            // This unsafe is really unsafe.
            // The stores list is NOT really &'static.
            // We must always check the lock before accessing it.
            // This is why use_ref's Reference can fail with SubtreeUnmountedError.
            transmute::<&'_ TreeStores<Ret, Props>, &'static TreeStores<Ret, Props>>(
                self.stores.borrow(),
            )
        };

        type StillFullNodeComponentStore<T> = ComponentConstruction<T, T, NoHoleNode, NoHoleNode>;
        let component_store: StillFullNodeComponentStore<_> = ComponentConstruction {
            hook_stores: HookConstruction {
                current: stores_borrow,
                entire: PhantomData,
                lock: self.lock.clone(),
                current_component: &DUMMY_ROOT,
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
    let stores = Box::new(stores);
    let lock = UnmountedLock::new_mounted();
    let document = window().unwrap().document().unwrap();
    let fragment = document.create_document_fragment();
    let parent = fragment.clone().into();
    let mut subtree = SubtreeInstance {
        stores,
        lock,
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
