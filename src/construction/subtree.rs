use std::{borrow::Borrow, marker::PhantomData, mem::transmute};
use web_sys::{window, Element, HtmlElement};

use crate::{
    stores::{StoreCons, StoresList},
    unmounted_lock::UnmountedLock,
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

When mounting the app [`crate::mount_app`], Reia creates a subtree for it.

[`crate::opt_comp`] creates a subtree for its component.
[`crate::vec_comps`] creates a subtree for each of its components.

A subtree wraps its children in a `<div style="display: contents">`.
Do take this into account when writing CSS selectors.

 */
pub struct SubtreeInstance<Ret, Props>
where
    Ret: ComponentReturn,
    Props: ComponentProps,
{
    stores: Box<TreeStores<Ret, Props>>,
    lock: UnmountedLock,
    container: Element,
    func: ComponentFunc<Ret, Props>,
}

impl<Ret, Props> Drop for SubtreeInstance<Ret, Props>
where
    Ret: ComponentReturn,
    Props: ComponentProps,
{
    fn drop(&mut self) {
        self.lock.unmount();
        self.container.remove();
    }
}

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
            // Always check the UnmountedLock before accessing it.
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
            parent_node: self.container.clone(),
            last_node: NoHoleNode,
            ret_node: NoHoleNode,
        };
        component_store.comp(self.func, props);
    }
}

pub(crate) fn create_wrapper_div() -> Element {
    use wasm_bindgen::JsCast;
    let document = window().unwrap().document().unwrap();
    let wrapper: HtmlElement = document.create_element("div").unwrap().dyn_into().unwrap();
    wrapper.style().set_property("display", "contents").unwrap();
    wrapper.into()
}

pub(crate) fn mount_subtree<Ret, Props>(
    func: ComponentFunc<Ret, Props>,
    props: Props,
    container: Element,
) -> SubtreeInstance<Ret, Props>
where
    Ret: ComponentReturn,
    Props: ComponentProps,
{
    let stores: TreeStores<Ret, Props> = StoresList::create();
    let stores = Box::new(stores);
    let lock = UnmountedLock::new_mounted();
    let subtree_root = create_wrapper_div();
    let subtree = SubtreeInstance {
        stores,
        lock,
        container: subtree_root.clone(),
        func,
    };
    subtree.re_render(props);
    container.append_child(&subtree_root).unwrap();
    subtree
}
