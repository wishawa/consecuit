use std::cell::RefCell;

use super::{
    component::{ComponentBuilder, ComponentConstruction, ComponentStoreInstance},
    hole::NoHoleNode,
    hook::{HookBuilder, HookConstruction},
    subtree::Subtree,
    types::{ComponentFunc, ComponentProps, ComponentReturn, HookReturn},
};
use crate::stores::{StoreCons, StoresList};

impl HookBuilder {
    /// This is a shortcut that calls `cc.init().hook(...)`
    ///
    /// It is here so you don't have to write `let cc = cc.init();` at the beginning of every hook function.
    ///
    /// See the docs at [`crate`] on how to call hooks.
    /// See [`HookConstruction`] for the actual `.hook(...)`.
    pub fn hook<Arg, Out, Ret, RestStores>(
        self,
        hook_func: fn(HookBuilder, Arg) -> Ret,
        hook_arg: Arg,
    ) -> (
        HookConstruction<RestStores, StoreCons<Ret::StoresList, RestStores>>,
        Out,
    )
    where
        RestStores: StoresList,
        Ret: HookReturn<Out>,
    {
        self.init().hook(hook_func, hook_arg)
    }
}

impl ComponentBuilder {
    /// This is a shortcut that calls `cc.init().hook(...)`
    ///
    /// It is here so you don't have to write `let cc = cc.init();` at the beginning of every component.
    ///
    /// See the docs at [`crate`] on how to call hooks.
    /// See [`ComponentConstruction`] for the actual `.hook(...)`.
    pub fn hook<Arg, Out, Ret, RestStores>(
        self,
        hook_func: fn(HookBuilder, Arg) -> Ret,
        hook_arg: Arg,
    ) -> (
        ComponentConstruction<
            RestStores,
            StoreCons<Ret::StoresList, RestStores>,
            NoHoleNode,
            NoHoleNode,
        >,
        Out,
    )
    where
        RestStores: StoresList,
        Ret: HookReturn<Out>,
    {
        self.init().hook(hook_func, hook_arg)
    }

    /// This is a shortcut that calls `cc.init().comp(...)`
    ///
    /// It is here so you don't have to write `let cc = cc.init();` at the beginning of every component.
    ///
    /// See the docs at [`crate`] on how to compose components.
    /// See [`ComponentConstruction`] for the actual `.comp(...)`.
    pub fn comp<Props, Ret, RestStores>(
        self,
        component_func: ComponentFunc<Ret, Props>,
        component_props: Props,
    ) -> ComponentConstruction<
        RestStores,
        StoreCons<ComponentStoreInstance<Ret, Props>, RestStores>,
        Ret::HoleNode,
        NoHoleNode,
    >
    where
        RestStores: StoresList,
        Ret: ComponentReturn,
        Props: ComponentProps,
    {
        self.init().comp(component_func, component_props)
    }

    /// This is a shortcut that calls `cc.init().dyn_comp(...)`
    ///
    /// It is here so you don't have to write `let cc = cc.init();` at the beginning of every component.
    ///
    /// See [`ComponentConstruction`] for the actual `.dyn_comp(...)` for more details.
    pub fn dyn_comp<Props, Ret, RestStores>(
        self,
        component_func: ComponentFunc<Ret, Props>,
        component_props: Props,
    ) -> ComponentConstruction<
        RestStores,
        StoreCons<RefCell<Option<Box<dyn Subtree<Props = Props>>>>, RestStores>,
        NoHoleNode,
        NoHoleNode,
    >
    where
        RestStores: StoresList,
        Ret: ComponentReturn,
        Props: ComponentProps,
    {
        self.init().dyn_comp(component_func, component_props)
    }
}
