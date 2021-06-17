use web_sys::window;

use crate::{ComponentBuilder, ComponentReturn, component::{create_subtree, ReiaSubtree}, hooks::{use_ref, ReiaRef}, props::PropsPartialEq};

pub fn dyn_opt_comp<Func, Ret, Props>(
    reia: ComponentBuilder,
    (func, props): (Func, Option<Props>),
) -> impl ComponentReturn
where
    Ret: ComponentReturn,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
    Props: PropsPartialEq + Clone + 'static,
{
    let reia = reia.init();
    let (reia, store): (_, ReiaRef<Option<ReiaSubtree<Func, Ret, Props>>>) = reia.hook(use_ref, ());
    store
        .visit_mut_with(|opt| {
            if let Some(props) = props {
                let subtree = opt.get_or_insert_with(|| {
                    let document = window().unwrap().document().unwrap();
                    let parent_node = document.create_element("div").unwrap();
                    reia.parent_node.append_child(&parent_node).unwrap();
                    create_subtree(parent_node)
                });
                subtree.run(func, props);
            } else {
                *opt = None;
            }
        })
        .unwrap();
    reia.bare_leaf_node()
}

pub fn dyn_vec_comps<Func, Ret, Props>(
    reia: ComponentBuilder,
    (func, props): (Func, Vec<Props>),
) -> impl ComponentReturn
where
    Ret: ComponentReturn,
    Func: Clone + 'static + Clone + Fn(ComponentBuilder, Props) -> Ret,
    Props: PropsPartialEq + Clone + 'static,
{
    let reia = reia.init();
    let (reia, store): (_, ReiaRef<Vec<ReiaSubtree<Func, Ret, Props>>>) = reia.hook(use_ref, ());
    store
        .visit_mut_with(|subtrees| {
            let current_length = subtrees.len();
            let new_length = props.len();
            if new_length > current_length {
                let document = window().unwrap().document().unwrap();
                subtrees.extend((current_length..new_length).map(|_| {
                    let parent_node = document.create_element("div").unwrap();
                    reia.parent_node.append_child(&parent_node).unwrap();
                    create_subtree(parent_node)
                }))
            } else if new_length < current_length {
                subtrees.truncate(new_length);
            }
            for (subtree, props) in subtrees.iter().zip(props.into_iter()) {
                subtree.run(func.clone(), props);
            }
        })
        .unwrap();
    reia.bare_leaf_node()
}