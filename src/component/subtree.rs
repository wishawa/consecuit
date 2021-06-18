use web_sys::{window, Element};

use crate::{
    component::{ComponentConstruction, ComponentStore, NoHoleNode},
    hook::HookConstruction,
    stores::{StoreCons, StoresList},
    unmounted_lock::UnmountedLock,
    ComponentBuilder, ComponentReturn,
};

use super::{ComponentFunc, ComponentProps, ComponentStoreInstance};
use std::{borrow::Borrow, marker::PhantomData, mem::transmute};

type TreeStores<Ret, Props> =
    StoreCons<ComponentStoreInstance<Ret, Props>, <Ret as ComponentReturn>::StoresList>;

pub struct SubtreeInstance<Ret, Props>
where
    Ret: ComponentReturn,
    Props: ComponentProps,
{
    stores: Box<TreeStores<Ret, Props>>,
    lock: UnmountedLock,
    container: Element,
    func: ComponentFunc<Props, Ret>,
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

pub(crate) trait Subtree {
    type Props;
    fn run(&self, props: Self::Props);
}

impl<Ret, Props> Subtree for SubtreeInstance<Ret, Props>
where
    Ret: ComponentReturn,
    Props: ComponentProps,
{
    type Props = Props;
    fn run(&self, props: Self::Props) {
        self.run(props)
    }
}

impl<Ret, Props> SubtreeInstance<Ret, Props>
where
    Ret: ComponentReturn,
    Props: ComponentProps,
{
    pub(crate) fn run(&self, props: Props) {
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
        component_store.comp(self.func.clone(), props);
    }
}

pub fn mount<Ret>(function: fn(ComponentBuilder, ()) -> Ret)
where
    Ret: ComponentReturn,
{
    let document = window().unwrap().document().unwrap();

    let app_root: Element = document.get_element_by_id("reia-app-root").unwrap();

    let root_tree = mount_subtree(function, (), app_root);
    root_tree.run(());

    Box::leak(Box::new(root_tree));
}

pub(crate) fn mount_subtree<Ret, Props>(
    func: ComponentFunc<Props, Ret>,
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
    let document = window().unwrap().document().unwrap();
    let parent_node: Element = document.create_element("div").unwrap();
    let subtree = SubtreeInstance {
        stores,
        lock,
        container: parent_node.clone(),
        func,
    };
    subtree.run(props);
    container.append_child(&parent_node).unwrap();
    subtree
}
