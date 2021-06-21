use std::{cell::RefCell, ops::DerefMut};

use im_rc::Vector;
use web_sys::Element;

use crate::{
    hooks::{use_ref, ReiaRef},
    stores::{StoreCons, StoresList},
    ComponentBuilder, ComponentReturn,
};

use super::{
    hole::{MaybeHoleNode, NoHoleNode},
    subtree::{create_wrapper_div, mount_subtree, Subtree, SubtreeInstance},
    types::{ComponentFunc, ComponentProps},
    ComponentConstruction,
};

fn get_or_create_container<'a>(opt: &'a mut Option<Element>, parent: Element) -> Element {
    let container = create_wrapper_div();
    opt.get_or_insert_with(|| {
        parent.append_child(&container).unwrap();
        container.clone()
    });
    container
}

pub fn opt_comp<Ret, Props>(
    reia: ComponentBuilder,
    (func, props): (ComponentFunc<Ret, Props>, Option<Props>),
) -> impl ComponentReturn
where
    Ret: ComponentReturn,
    Props: ComponentProps,
{
    let reia = reia.init();
    let (reia, container): (_, ReiaRef<Option<Element>>) = reia.hook(use_ref, ());
    let (reia, subtree): (_, ReiaRef<Option<SubtreeInstance<Ret, Props>>>) = reia.hook(use_ref, ());
    let parent = container
        .visit_mut_with(|container_opt| {
            get_or_create_container(container_opt, reia.get_parent_node())
        })
        .unwrap();

    subtree
        .visit_mut_with(|subtree_opt| {
            if let Some(props) = props {
                match subtree_opt {
                    Some(subtree) => subtree.re_render(props),
                    component_opt_none => {
                        *component_opt_none = Some(mount_subtree(func, props, parent.clone()));
                    }
                }
            } else {
                *subtree_opt = None;
            }
        })
        .unwrap();
    reia.bare_leaf_node()
}

pub fn vec_comps<Ret, Props>(
    reia: ComponentBuilder,
    (func, mut props): (ComponentFunc<Ret, Props>, Vector<Props>),
) -> impl ComponentReturn
where
    Ret: ComponentReturn,
    Props: ComponentProps,
{
    let reia = reia.init();
    let (reia, container): (_, ReiaRef<Option<Element>>) = reia.hook(use_ref, ());
    let (reia, subtree): (_, ReiaRef<Vec<SubtreeInstance<Ret, Props>>>) = reia.hook(use_ref, ());
    let parent = container
        .visit_mut_with(|container_opt| {
            get_or_create_container(container_opt, reia.get_parent_node())
        })
        .unwrap();
    subtree
        .visit_mut_with(|subtrees| {
            let current_length = subtrees.len();
            let new_length = props.len();

            if new_length > current_length {
                let new_props = props.split_off(current_length);
                subtrees.extend(
                    new_props
                        .into_iter()
                        .map(|prop| mount_subtree(func, prop, parent.clone())),
                )
            } else {
                subtrees.truncate(new_length);
            }
            for (subtree, props) in subtrees.iter().zip(props.into_iter()) {
                subtree.re_render(props);
            }
        })
        .unwrap();
    reia.bare_leaf_node()
}

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
        func: ComponentFunc<Ret, Props>,
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
            Some(subtree) => subtree.re_render(props),
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
