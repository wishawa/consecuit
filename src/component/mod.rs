use crate::{
    executor::{Renderable, RerenderTask},
    hook::{HookBuilder, HookReturn, HookStores},
    stores::{StoreCons, StoreConsEnd, StoresList},
    unmounted_lock::UnmountedLock,
};
use std::{cell::RefCell, marker::PhantomData, mem::transmute, ops::DerefMut};
use web_sys::Element;
pub mod utils;
use utils::{ComponentFunc, ComponentProps};

use self::hole::{MaybeHoleNode, NoHoleNode, YesHoleNode};
mod hole;

pub mod bare;
pub mod subtree;
pub use subtree::mount;

pub struct ComponentBuilder {
    pub(crate) hook_builder: HookBuilder,
    pub(crate) parent_node: Element,
}

impl ComponentBuilder {
    pub fn init<T: StoresList>(self) -> ComponentStores<T, T, NoHoleNode, NoHoleNode> {
        let Self {
            hook_builder,
            parent_node,
        } = self;
        ComponentStores {
            hook_stores: hook_builder.init::<T>(),
            parent_node,
            last_node: NoHoleNode,
            ret_node: NoHoleNode,
        }
    }
}

pub trait ComponentReturn: 'static {
    type StoresList: StoresList;
    type HoleNode: MaybeHoleNode;
    fn get_node(self) -> Self::HoleNode;
}

pub trait ContainerReturn: ComponentReturn<HoleNode = YesHoleNode> {}
impl<T: ComponentReturn<HoleNode = YesHoleNode>> ContainerReturn for T {}

impl<Stores: StoresList, LastNode: MaybeHoleNode, HoleNode: MaybeHoleNode> ComponentReturn
    for ComponentStores<StoreConsEnd, Stores, LastNode, HoleNode>
{
    type StoresList = Stores;
    type HoleNode = HoleNode;
    fn get_node(self) -> Self::HoleNode {
        self.ret_node
    }
}

pub struct ComponentStores<
    CurrentStores: StoresList,
    EntireStores: StoresList,
    LastNode: MaybeHoleNode,
    ReturnNode: MaybeHoleNode,
> {
    pub(crate) hook_stores: HookStores<CurrentStores, EntireStores>,
    pub(crate) parent_node: Element,
    pub(crate) last_node: LastNode,
    pub(crate) ret_node: ReturnNode,
}

impl<ThisStore, RestStores, EntireStores, LastNode, CompHole>
    ComponentStores<StoreCons<ThisStore, RestStores>, EntireStores, LastNode, CompHole>
where
    ThisStore: StoresList,
    RestStores: StoresList,
    EntireStores: StoresList,
    LastNode: MaybeHoleNode,
    CompHole: MaybeHoleNode,
{
    pub fn hook<Arg, Ret, Out>(
        self,
        hook_func: fn(HookBuilder, Arg) -> Ret,
        hook_arg: Arg,
    ) -> (
        ComponentStores<RestStores, EntireStores, LastNode, CompHole>,
        Out,
    )
    where
        Ret: HookReturn<Out, StoresList = ThisStore>,
    {
        let ComponentStores {
            hook_stores,
            parent_node: node,
            last_node,
            ret_node,
        } = self;
        let (hook_stores, out) = hook_stores.hook(hook_func, hook_arg);
        let comp_stores = ComponentStores {
            hook_stores,
            parent_node: node,
            last_node,
            ret_node,
        };
        (comp_stores, out)
    }
}

pub struct ComponentContainer<Ret, Props>
where
    Props: 'static,
    Ret: ComponentReturn,
{
    stores: Ret::StoresList,
    initialized: RefCell<Option<InitializedComponentInfo<Ret, Props>>>,
}

struct InitializedComponentInfo<Ret, Props>
where
    Props: 'static,
    Ret: ComponentReturn,
{
    func: ComponentFunc<Props, Ret>,
    props: Props,
    parent_node: Element,
    my_hole: Option<Ret::HoleNode>,
    lock: UnmountedLock,
}

impl<Ret, Props> Default for ComponentContainer<Ret, Props>
where
    Props: 'static,
    Ret: ComponentReturn,
{
    fn default() -> Self {
        Self {
            stores: Ret::StoresList::create(),
            initialized: RefCell::new(None),
        }
    }
}

impl<Ret, Props> Renderable for ComponentContainer<Ret, Props>
where
    Props: 'static + Clone,
    Ret: ComponentReturn,
{
    fn render(self: &'static Self) {
        fn run_component<Props, Ret>(
            store: &'static Ret::StoresList,
            lock: UnmountedLock,
            rerender: RerenderTask,
            component_func: ComponentFunc<Props, Ret>,
            props: Props,
            on_node: Element,
        ) -> Ret::HoleNode
        where
            Ret: ComponentReturn,
        {
            let untyped_stores =
                unsafe { transmute::<&'static Ret::StoresList, &'static ()>(store) };
            let reia = ComponentBuilder {
                hook_builder: HookBuilder {
                    untyped_stores,
                    lock,
                    rerender_parent: rerender,
                },
                parent_node: on_node,
            };
            let holes = component_func(reia, props).get_node();
            holes
        }

        let mut ran_borrow = self.initialized.borrow_mut();
        let stores = &self.stores;
        let InitializedComponentInfo {
            func,
            props,
            my_hole,
            lock,
            parent_node,
        } = ran_borrow.deref_mut().as_mut().unwrap();
        let rerender = RerenderTask {
            obj: self,
            lock: lock.clone(),
        };
        *my_hole = Some(run_component(
            stores,
            lock.clone(),
            rerender,
            func.clone(),
            props.clone(),
            parent_node.clone(),
        ));
    }
}

impl<RestStores, EntireStores, Ret, Props, LastNode, CompHole>
    ComponentStores<
        StoreCons<ComponentContainer<Ret, Props>, RestStores>,
        EntireStores,
        LastNode,
        CompHole,
    >
where
    RestStores: StoresList,
    EntireStores: StoresList,
    LastNode: MaybeHoleNode,
    CompHole: MaybeHoleNode,
    Ret: ComponentReturn,
    Props: PartialEq + Clone,
{
    pub fn node(
        self,
        component_func: ComponentFunc<Props, Ret>,
        component_props: Props,
    ) -> ComponentStores<RestStores, EntireStores, Ret::HoleNode, CompHole> {
        let ComponentStores {
            hook_stores,
            parent_node,
            ret_node,
            ..
        } = self;
        let (rests, container_store) = hook_stores.use_one_store();
        let should_render = {
            let mut ran_borrow = container_store.initialized.borrow_mut();
            match ran_borrow.deref_mut() {
                Some(ran_info) => {
                    let props = &mut ran_info.props;
                    if component_props == *props {
                        false
                    } else {
                        *props = component_props.clone();
                        true
                    }
                }
                opt_none => {
                    *opt_none = Some(InitializedComponentInfo {
                        func: component_func,
                        props: component_props,
                        my_hole: None,
                        lock: rests.lock.clone(),
                        parent_node: parent_node.clone(),
                    });
                    true
                }
            }
        };
        if should_render {
            container_store.render();
        }
        let last_node = container_store
            .initialized
            .borrow()
            .as_ref()
            .unwrap()
            .my_hole
            .clone()
            .unwrap();
        ComponentStores {
            hook_stores: rests,
            parent_node,
            ret_node,
            last_node,
        }
    }
}

impl<ThisStore, RestStores, EntireStores, CompHole>
    ComponentStores<StoreCons<ThisStore, RestStores>, EntireStores, YesHoleNode, CompHole>
where
    ThisStore: StoresList,
    RestStores: StoresList,
    EntireStores: StoresList,
    CompHole: MaybeHoleNode,
{
    pub fn child<Builder, ChildLastNode, ChildHole>(
        self,
        builder: Builder,
    ) -> ComponentStores<RestStores, EntireStores, NoHoleNode, ChildHole>
    where
        ChildHole: MaybeHoleNode,
        ChildLastNode: MaybeHoleNode,
        Builder: FnOnce(
            ComponentStores<ThisStore, ThisStore, NoHoleNode, CompHole>,
        )
            -> ComponentStores<StoreConsEnd, ThisStore, ChildLastNode, ChildHole>,
    {
        let ComponentStores {
            hook_stores,
            parent_node,
            ret_node,
            last_node,
        } = self;
        let (rest_stores, store) = hook_stores.use_one_store();
        let comp_stores = ComponentStores {
            hook_stores: HookStores {
                current: store,
                entire: PhantomData,
                lock: rest_stores.lock.clone(),
                rerender_parent: rest_stores.rerender_parent.clone(),
            },
            parent_node: last_node.0,
            ret_node,
            last_node: NoHoleNode,
        };
        let built = builder(comp_stores);
        ComponentStores {
            hook_stores: rest_stores,
            parent_node,
            last_node: NoHoleNode,
            ret_node: built.ret_node,
        }
    }
}
