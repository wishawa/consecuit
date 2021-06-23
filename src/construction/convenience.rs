use crate::stores::{StoreCons, StoresList};

use super::types::HookReturn;
use super::{
    hook::{HookBuilder, HookConstruction},
    subtree::Subtree,
};

impl HookBuilder {
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

use std::cell::RefCell;

use super::component::{ComponentBuilder, ComponentConstruction, ComponentStoreInstance};
use super::hole::NoHoleNode;
use super::types::{ComponentFunc, ComponentProps, ComponentReturn};

impl ComponentBuilder {
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
