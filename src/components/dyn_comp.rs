use web_sys::window;

use crate::{
    component::{
        subtree::{create_subtree, ReiaSubtree, Subtree},
        utils::{ComponentFunc, ComponentProps},
    },
    hooks::{use_ref, ReiaRef},
    ComponentBuilder, ComponentReturn,
};

pub fn opt_comp<Ret, Props>(
    reia: ComponentBuilder,
    (func, props): (ComponentFunc<Props, Ret>, Option<Props>),
) -> impl ComponentReturn
where
    Ret: ComponentReturn,
    Props: ComponentProps,
{
    let reia = reia.init();
    let (reia, store): (_, ReiaRef<Option<ReiaSubtree<Ret, Props>>>) = reia.hook(use_ref, ());
    store
        .visit_mut_with(|opt| {
            if let Some(props) = props {
                let subtree = opt.get_or_insert_with(|| {
                    let document = window().unwrap().document().unwrap();
                    let parent_node = document.create_element("div").unwrap();
                    reia.parent_node.append_child(&parent_node).unwrap();
                    create_subtree(func, parent_node)
                });
                subtree.run(props);
            } else {
                *opt = None;
            }
        })
        .unwrap();
    reia.bare_leaf_node()
}

pub fn dyn_opt_comp<Ret, Props>(
    reia: ComponentBuilder,
    (func, props): (ComponentFunc<Props, Ret>, Option<Props>),
) -> impl ComponentReturn
where
    Ret: ComponentReturn,
    Props: ComponentProps,
{
    let reia = reia.init();
    let (reia, store): (_, ReiaRef<Option<Box<dyn Subtree<Props = Props>>>>) =
        reia.hook(use_ref, ());
    store
        .visit_mut_with(|opt| {
            if let Some(props) = props {
                let subtree = opt.get_or_insert_with(|| {
                    let document = window().unwrap().document().unwrap();
                    let parent_node = document.create_element("div").unwrap();
                    reia.parent_node.append_child(&parent_node).unwrap();
                    Box::new(create_subtree(func, parent_node)) as Box<dyn Subtree<Props = Props>>
                });
                subtree.run(props);
            } else {
                *opt = None;
            }
        })
        .unwrap();
    reia.bare_leaf_node()
}

pub fn dyn_vec_comps<Ret, Props>(
    reia: ComponentBuilder,
    (func, props): (ComponentFunc<Props, Ret>, Vec<Props>),
) -> impl ComponentReturn
where
    Ret: ComponentReturn,
    Props: ComponentProps,
{
    let reia = reia.init();
    let (reia, store): (_, ReiaRef<Vec<Box<dyn Subtree<Props = Props>>>>) = reia.hook(use_ref, ());
    store
        .visit_mut_with(|subtrees| {
            let current_length = subtrees.len();
            let new_length = props.len();
            if new_length > current_length {
                let document = window().unwrap().document().unwrap();
                subtrees.extend((current_length..new_length).map(|_| {
                    let parent_node = document.create_element("div").unwrap();
                    reia.parent_node.append_child(&parent_node).unwrap();
                    Box::new(create_subtree(func, parent_node)) as Box<dyn Subtree<Props = Props>>
                }))
            } else if new_length < current_length {
                subtrees.truncate(new_length);
            }
            for (subtree, props) in subtrees.iter().zip(props.into_iter()) {
                subtree.run(props);
            }
        })
        .unwrap();
    reia.bare_leaf_node()
}
