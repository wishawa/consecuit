use crate::{
    stores::{StoreCons, StoreConsEnd, StoresList},
    unmounted_lock::UnmountedLock,
};
use std::{cell::RefCell, marker::PhantomData, ops::DerefMut};
use web_sys::Element;

use super::{
    hole::{MaybeHoleNode, NoHoleNode, YesHoleNode},
    hook::{HookBuilder, HookConstruction},
    types::{ComponentFunc, ComponentProps, ComponentReturn, HookReturn},
};

pub struct ComponentBuilder {
    pub(crate) hook_builder: HookBuilder,
    pub(crate) parent_node: Element,
}

impl ComponentBuilder {
    pub fn init<T: StoresList>(self) -> ComponentConstruction<T, T, NoHoleNode, NoHoleNode> {
        let Self {
            hook_builder,
            parent_node,
        } = self;
        ComponentConstruction {
            hook_stores: hook_builder.init::<T>(),
            parent_node,
            last_node: NoHoleNode,
            ret_node: NoHoleNode,
        }
    }
}

pub struct ComponentConstruction<
    CurrentStores: StoresList,
    EntireStores: StoresList,
    LastNode: MaybeHoleNode,
    ReturnNode: MaybeHoleNode,
> {
    pub(crate) hook_stores: HookConstruction<CurrentStores, EntireStores>,
    pub(crate) parent_node: Element,
    pub(crate) last_node: LastNode,
    pub(crate) ret_node: ReturnNode,
}

impl<CurrentStores, EntireStores, LastNode, CompHole>
    ComponentConstruction<CurrentStores, EntireStores, LastNode, CompHole>
where
    CurrentStores: StoresList,
    EntireStores: StoresList,
    LastNode: MaybeHoleNode,
    CompHole: MaybeHoleNode,
{
    pub fn get_parent_node(&self) -> Element {
        self.parent_node.clone()
    }
}

impl<CurrentStores, RestStores, EntireStores, LastNode, CompHole>
    ComponentConstruction<StoreCons<CurrentStores, RestStores>, EntireStores, LastNode, CompHole>
where
    CurrentStores: StoresList,
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
        ComponentConstruction<RestStores, EntireStores, LastNode, CompHole>,
        Out,
    )
    where
        Ret: HookReturn<Out, StoresList = CurrentStores>,
    {
        let ComponentConstruction {
            hook_stores,
            parent_node: node,
            last_node,
            ret_node,
        } = self;
        let (hook_stores, out) = hook_stores.hook(hook_func, hook_arg);
        let comp_stores = ComponentConstruction {
            hook_stores,
            parent_node: node,
            last_node,
            ret_node,
        };
        (comp_stores, out)
    }
}

pub struct ComponentStoreInstance<Ret, Props>
where
    Props: ComponentProps,
    Ret: ComponentReturn,
{
    stores: Ret::StoresList,
    initialized: RefCell<Option<InitializedComponentInfo<Ret, Props>>>,
}

pub(crate) trait ComponentStore {
    fn render(&'static self);
}

struct InitializedComponentInfo<Ret, Props>
where
    Props: ComponentProps,
    Ret: ComponentReturn,
{
    func: ComponentFunc<Ret, Props>,
    props: Props,
    parent_node: Element,
    my_hole: Option<Ret::HoleNode>,
    lock: UnmountedLock,
}

impl<Ret, Props> Default for ComponentStoreInstance<Ret, Props>
where
    Props: ComponentProps,
    Ret: ComponentReturn,
{
    fn default() -> Self {
        Self {
            stores: Ret::StoresList::create(),
            initialized: RefCell::new(None),
        }
    }
}

impl<Ret, Props> ComponentStore for ComponentStoreInstance<Ret, Props>
where
    Props: ComponentProps,
    Ret: ComponentReturn,
{
    fn render(&'static self) {
        let mut ran_borrow = self.initialized.borrow_mut();
        let stores = &self.stores;
        let InitializedComponentInfo {
            func,
            props,
            my_hole,
            lock,
            parent_node,
        } = ran_borrow.deref_mut().as_mut().unwrap();

        let untyped_stores: *const () =
            stores as *const <Ret as ComponentReturn>::StoresList as *const ();
        let reia = ComponentBuilder {
            hook_builder: HookBuilder {
                untyped_stores,
                lock: lock.clone(),
                current_component: self,
            },
            parent_node: parent_node.clone(),
        };
        let hole = func(reia, props.clone()).get_node();

        *my_hole = Some(hole);
    }
}

impl<RestStores, EntireStores, Ret, Props, LastNode, CompHole>
    ComponentConstruction<
        StoreCons<ComponentStoreInstance<Ret, Props>, RestStores>,
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
    Props: ComponentProps,
{
    pub fn comp(
        self,
        component_func: ComponentFunc<Ret, Props>,
        component_props: Props,
    ) -> ComponentConstruction<RestStores, EntireStores, Ret::HoleNode, CompHole> {
        let ComponentConstruction {
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
                        *props = component_props;
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
        ComponentConstruction {
            hook_stores: rests,
            parent_node,
            ret_node,
            last_node,
        }
    }
}

impl<CurrentStores, RestStores, EntireStores, CompHole>
    ComponentConstruction<StoreCons<CurrentStores, RestStores>, EntireStores, YesHoleNode, CompHole>
where
    CurrentStores: StoresList,
    RestStores: StoresList,
    EntireStores: StoresList,
    CompHole: MaybeHoleNode,
{
    pub fn child<Builder, ChildLastNode, ChildHole>(
        self,
        builder: Builder,
    ) -> ComponentConstruction<RestStores, EntireStores, NoHoleNode, ChildHole>
    where
        ChildHole: MaybeHoleNode,
        ChildLastNode: MaybeHoleNode,
        Builder:
            FnOnce(
                ComponentConstruction<CurrentStores, CurrentStores, NoHoleNode, CompHole>,
            )
                -> ComponentConstruction<StoreConsEnd, CurrentStores, ChildLastNode, ChildHole>,
    {
        let ComponentConstruction {
            hook_stores,
            parent_node,
            ret_node,
            last_node,
        } = self;
        let (rest_stores, store) = hook_stores.use_one_store();
        let comp_stores = ComponentConstruction {
            hook_stores: HookConstruction {
                current: store,
                entire: PhantomData,
                lock: rest_stores.lock.clone(),
                current_component: rest_stores.current_component,
            },
            parent_node: last_node.0,
            ret_node,
            last_node: NoHoleNode,
        };
        let built = builder(comp_stores);
        ComponentConstruction {
            hook_stores: rest_stores,
            parent_node,
            last_node: NoHoleNode,
            ret_node: built.ret_node,
        }
    }
}
