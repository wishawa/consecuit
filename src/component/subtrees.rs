use std::{cell::RefCell, ops::DerefMut};

use crate::{
    stores::{StoreCons, StoresList},
    ComponentReturn,
};

use super::{
    hole::{MaybeHoleNode, NoHoleNode},
    subtree::{mount_subtree, Subtree, SubtreeInstance},
    utils::{ComponentFunc, ComponentProps},
    ComponentConstruction,
};

impl<RestStores, EntireStores, Props, LastNode, CompHole>
    ComponentConstruction<
        StoreCons<RefCell<Option<Box<dyn Subtree<Props = Props>>>>, RestStores>,
        EntireStores,
        LastNode,
        CompHole,
    >
where
    RestStores: StoresList,
    EntireStores: StoresList,
    LastNode: MaybeHoleNode,
    CompHole: MaybeHoleNode,
    Props: ComponentProps,
{
    pub fn dyn_comp<Ret: ComponentReturn>(
        self,
        func: ComponentFunc<Props, Ret>,
        props: Props,
    ) -> ComponentConstruction<RestStores, EntireStores, NoHoleNode, CompHole> {
        let ComponentConstruction {
            hook_stores,
            parent_node,
            ret_node,
            ..
        } = self;
        let (rest_hooks, store) = hook_stores.use_one_store();
        let mut store_borrow = store.borrow_mut();
        let subtree_opt = store_borrow.deref_mut();
        match subtree_opt {
            Some(subtree) => subtree.run(props),
            opt_none => {
                *opt_none = Some(Box::new(mount_subtree(func, props, parent_node.clone())));
            }
        }
        ComponentConstruction {
            hook_stores: rest_hooks,
            parent_node,
            ret_node,
            last_node: NoHoleNode,
        }
    }
}

impl<RestStores, EntireStores, Ret, Props, LastNode, CompHole>
    ComponentConstruction<
        StoreCons<RefCell<Option<SubtreeInstance<Ret, Props>>>, RestStores>,
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
    pub fn opt_comp(
        self,
        func: ComponentFunc<Props, Ret>,
        props: Option<Props>,
    ) -> ComponentConstruction<RestStores, EntireStores, NoHoleNode, CompHole> {
        let ComponentConstruction {
            hook_stores,
            parent_node,
            ret_node,
            ..
        } = self;
        let (rest_hooks, store) = hook_stores.use_one_store();
        let mut store_borrow = store.borrow_mut();
        let subtree_opt = store_borrow.deref_mut();
        if let Some(props) = props {
            match subtree_opt {
                Some(subtree) => subtree.run(props),
                opt_none => {
                    *opt_none = Some(mount_subtree(func, props, parent_node.clone()));
                }
            }
        } else {
            *subtree_opt = None;
        }
        ComponentConstruction {
            hook_stores: rest_hooks,
            parent_node,
            ret_node,
            last_node: NoHoleNode,
        }
    }
}

impl<RestStores, EntireStores, Ret, Props, LastNode, CompHole>
    ComponentConstruction<
        StoreCons<RefCell<Vec<SubtreeInstance<Ret, Props>>>, RestStores>,
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
    pub fn vec_comps(
        self,
        func: ComponentFunc<Props, Ret>,
        mut props: Vec<Props>,
    ) -> ComponentConstruction<RestStores, EntireStores, NoHoleNode, CompHole> {
        let ComponentConstruction {
            hook_stores,
            parent_node,
            ret_node,
            ..
        } = self;
        let (rest_hooks, store) = hook_stores.use_one_store();
        let mut store_borrow = store.borrow_mut();
        let subtrees = store_borrow.deref_mut();
        let current_length = subtrees.len();
        let new_length = props.len();

        if new_length > current_length {
            let new_props = props.split_off(current_length);
            subtrees.extend(
                new_props
                    .into_iter()
                    .map(|prop| mount_subtree(func, prop, parent_node.clone())),
            )
        } else {
            subtrees.truncate(new_length);
        }
        for (subtree, props) in subtrees.iter().zip(props.into_iter()) {
            subtree.run(props);
        }
        ComponentConstruction {
            hook_stores: rest_hooks,
            parent_node,
            ret_node,
            last_node: NoHoleNode,
        }
    }
}
