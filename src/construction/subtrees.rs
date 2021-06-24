use std::{cell::RefCell, ops::DerefMut};

use im_rc::Vector;
use web_sys::Element;

use crate::{
    hooks::{use_ref, Reference},
    stores::{StoreCons, StoresList},
};

use super::{
    component::{ComponentBuilder, ComponentConstruction},
    hole::{MaybeHoleNode, NoHoleNode},
};

use super::{
    subtree::{create_wrapper_div, mount_subtree, Subtree, SubtreeInstance},
    types::{ComponentFunc, ComponentProps, ComponentReturn},
};

fn get_or_create_container(opt: &mut Option<Element>, parent: Element) -> Element {
    opt.get_or_insert_with(|| {
        let container = create_wrapper_div();
        parent.append_child(&container).unwrap();
        container
    })
    .clone()
}

/** A wrapper component that monuts/unmounts the given component based on the `Option<Props>`.

Takes a tuple of `(the component function, Option<its props>)`.

Component is mounted when props is Some(props).
Component is unmouned when props is None.

Component loses all state when unmounted.

Use like this

```
cc_tree!(
    <opt_comp {(my_comp, Some(my_comp_props))} />
    <opt_comp {(other_comp, None)} />
)
```

Component is mounted/unmouned as its own [Subtree][SubtreeInstance].
*/
pub fn opt_comp<Ret, Props>(
    cc: ComponentBuilder,
    (func, props): (ComponentFunc<Ret, Props>, Option<Props>),
) -> impl ComponentReturn
where
    Ret: ComponentReturn,
    Props: ComponentProps,
{
    let cc = cc.init();
    let (cc, container): (_, Reference<Option<Element>>) = cc.hook(use_ref, ());
    let (cc, subtree): (_, Reference<Option<SubtreeInstance<Ret, Props>>>) = cc.hook(use_ref, ());
    let parent = container
        .visit_mut_with(|container_opt| {
            get_or_create_container(container_opt, cc.get_parent_node())
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
    cc.bare_leaf_node()
}

/** A wrapper component  that monuts/unmounts a number of the given component based on the `Vector<Props>`.

Note that this takes a [`im_rc::Vector`], not a [`std::vec::Vec`].

Takes a tuple of `(the component function, Vector<its props>)`.

For each item in the vector, a component is mounted with the item as props.

If the vector grows, more components are mounted.
If the vector shrinks, some components are dismounted.

**A note of caution:** Component number i gets props number i.
Shifting part of the `Vector` won't shift the components' states.
This is like keying React components in a loop with the index.

Components loses all state when unmounted.

Use like this

```
cc_tree!(
    <vec_comps {(my_comp, vector![props1, props2, props3])} />
)
```

Each component is mounted/unmouned as its own [Subtree][SubtreeInstance].
*/

pub fn vec_comps<Ret, Props>(
    cc: ComponentBuilder,
    (func, mut props): (ComponentFunc<Ret, Props>, Vector<Props>),
) -> impl ComponentReturn
where
    Ret: ComponentReturn,
    Props: ComponentProps,
{
    let cc = cc.init();
    let (cc, container): (_, Reference<Option<Element>>) = cc.hook(use_ref, ());
    let (cc, subtree): (_, Reference<Vec<SubtreeInstance<Ret, Props>>>) = cc.hook(use_ref, ());
    let parent = container
        .visit_mut_with(|container_opt| {
            get_or_create_container(container_opt, cc.get_parent_node())
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
    cc.bare_leaf_node()
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
    /** Like `.comp(...)`, but mount the component in a dynamic subtree.

    This hides the components' typing from its parent.

    You can use this to name a component's return type concretely.

    The task is a bit boilerplatey. Copy paste this code:

    ```
    fn outer_comp(cc: ComponentBuilder, props: MyProp) -> DynComponentReturn<MyProp> {
        fn inner_comp(cc: ComponentBuilder, props: MyProp) -> impl ComponentReturn {
            cc_tree!(
                <div>
                    "hello, I'm a dynamic component. My props is:"
                    {props.to_string()}
                </div>
            )
        }
        consecuit.dyn_comp(inner_comp, props)
        // inner_comp doesn't have to be an inner function. a closure works too.
    }
    ```

    Then modify `inner_comp` as you like.
     */
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
