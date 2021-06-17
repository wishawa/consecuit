use web_sys::Element;

use crate::{
    component::{ComponentStores, NoHoleNode},
    executor::{Renderable, RerenderTask},
    hook::HookStores,
    stores::{StoreCons, StoresList},
    unmounted_lock::UnmountedLock,
    ComponentReturn,
};

use super::{ComponentContainer, ComponentFunc, ComponentProps};
use std::{borrow::Borrow, marker::PhantomData, mem::transmute};

type TreeStores<Ret, Props> =
    StoreCons<ComponentContainer<Ret, Props>, <Ret as ComponentReturn>::StoresList>;

pub(crate) struct ReiaSubtree<Ret, Props>
where
    Ret: ComponentReturn,
    Props: ComponentProps,
{
    stores: Box<TreeStores<Ret, Props>>,
    lock: UnmountedLock,
    container: Element,
    func: ComponentFunc<Props, Ret>,
}

impl<Ret, Props> Drop for ReiaSubtree<Ret, Props>
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

impl<Ret, Props> Subtree for ReiaSubtree<Ret, Props>
where
    Ret: ComponentReturn,
    Props: ComponentProps,
{
    type Props = Props;
    fn run(&self, props: Self::Props) {
        self.run(props)
    }
}

impl<Ret, Props> ReiaSubtree<Ret, Props>
where
    Ret: ComponentReturn,
    Props: ComponentProps,
{
    pub(crate) fn run(&self, props: Props) {
        struct PanicRootRerunner();
        impl Renderable for PanicRootRerunner {
            fn render(&'static self) {
                unreachable!("this dummy is never directly called")
            }
        }
        static PANIC_RERUNNER: PanicRootRerunner = PanicRootRerunner();

        let stores_borrow = unsafe {
            // This unsafe is really unsafe.
            // The stores list is NOT really &'static.
            // Always check the UnmountedLock before accessing it.
            transmute::<&'_ TreeStores<Ret, Props>, &'static TreeStores<Ret, Props>>(
                self.stores.borrow(),
            )
        };

        type StillFullNodeComponentStore<T> = ComponentStores<T, T, NoHoleNode, NoHoleNode>;
        let component_store: StillFullNodeComponentStore<_> = ComponentStores {
            hook_stores: HookStores {
                current: stores_borrow,
                entire: PhantomData,
                lock: self.lock.clone(),
                rerender_parent: RerenderTask {
                    obj: &PANIC_RERUNNER,
                    lock: self.lock.clone(),
                },
            },
            parent_node: self.container.clone(),
            last_node: NoHoleNode,
            ret_node: NoHoleNode,
        };
        component_store.node(self.func.clone(), props);
    }
}

pub(crate) fn create_subtree<Ret, Props>(
    func: ComponentFunc<Props, Ret>,
    container: Element,
) -> ReiaSubtree<Ret, Props>
where
    Ret: ComponentReturn,
    Props: ComponentProps,
{
    let stores: TreeStores<Ret, Props> = StoresList::create();
    let stores = Box::new(stores);
    let lock = UnmountedLock::new_mounted();
    ReiaSubtree {
        stores,
        lock,
        container,
        func,
    }
}