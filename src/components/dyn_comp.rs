use crate::{
    component::{
        subtree::{mount_subtree, Subtree, SubtreeInstance},
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
    let (reia, store): (_, ReiaRef<Option<SubtreeInstance<Ret, Props>>>) = reia.hook(use_ref, ());
    store
        .visit_mut_with(|opt| {
            if let Some(props) = props {
                match opt {
                    Some(subtree) => {
                        subtree.run(props);
                    }
                    opt_none => {
                        *opt_none = Some(mount_subtree(func, props, reia.parent_node.clone()));
                    }
                }
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
                match opt {
                    Some(subtree) => {
                        subtree.run(props);
                    }
                    opt_none => {
                        *opt_none = Some(Box::new(mount_subtree(
                            func,
                            props,
                            reia.parent_node.clone(),
                        )));
                    }
                }
            } else {
                *opt = None;
            }
        })
        .unwrap();
    reia.bare_leaf_node()
}

pub fn dyn_vec_comps<Ret, Props>(
    reia: ComponentBuilder,
    (func, mut props): (ComponentFunc<Props, Ret>, Vec<Props>),
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
                let new_props = props.split_off(current_length);
                subtrees.extend(new_props.into_iter().map(|prop| {
                    Box::new(mount_subtree(func, prop, reia.parent_node.clone()))
                        as Box<dyn Subtree<Props = Props>>
                }))
            } else {
                subtrees.truncate(new_length);
            }
            for (subtree, props) in subtrees.iter().zip(props.into_iter()) {
                subtree.run(props);
            }
        })
        .unwrap();
    reia.bare_leaf_node()
}
