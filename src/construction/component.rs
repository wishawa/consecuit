use crate::{
    locking::SharedPart,
    stores::{StoreCons, StoreConsEnd, StoresList},
};
use std::{cell::RefCell, marker::PhantomData, ops::DerefMut};
use web_sys::Node;

use super::{
    hole::{MaybeHoleNode, NoHoleNode, YesHoleNode},
    hook::{HookBuilder, HookConstruction},
    types::{ComponentFunc, ComponentProps, ComponentReturn, HookReturn},
};

/** The initial `consecuit` or `cc` object every component takes as first argument.

For more information on how to write components, see the docs at [crate].

*/
pub struct ComponentBuilder {
    pub(crate) hook_builder: HookBuilder,
    pub(crate) parent_node: Node,
}

impl ComponentBuilder {
    /// Make it ready to call `.hook(...)`, `.comp(...)`.
    /// You shouldn't need this, as we have a shortbut that automatically call it when you call `.hook(...)`, `.comp(...)`.
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

/** This is the `consecuit` or `cc` object in your component function.

You can use it to call hooks and render other components.

See the doc at [`crate`] on how to write components.
 */
pub struct ComponentConstruction<
    CurrentStores: StoresList,
    EntireStores: StoresList,
    LastNode: MaybeHoleNode,
    ReturnNode: MaybeHoleNode,
> {
    pub(crate) hook_stores: HookConstruction<CurrentStores, EntireStores>,
    pub(crate) parent_node: Node,
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
    /// Get the parent [Node] the current component should render on.
    ///
    /// This is for creating your own base component.
    /// If you stick with the ones provided by the `consecuit_html` crate, you won't need this.
    ///
    /// If you want to use this, see `consecuit_html`'s source code as example.
    pub fn get_parent_node(&self) -> Node {
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
    /** Use the given hook, with the given arg.

    Consumes `self`. Returns a tuple of `(cc, <return value of hook>)`.
    You can use the returned `cc` to call more hooks.

    See the docs at [crate] for more info on how to write and use hooks.
     */
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

/// Internal use
pub struct ComponentStoreInstance<Ret, Props>
where
    Props: ComponentProps,
    Ret: ComponentReturn,
{
    stores: Ret::StoresList,
    initialized: RefCell<Option<InitializedComponentInfo<Ret, Props>>>,
}

impl<Ret, Props> ComponentStore for SharedPart<ComponentStoreInstance<Ret, Props>>
where
    Props: ComponentProps,
    Ret: ComponentReturn,
{
    fn render(&self) {
        let mut bm = self.initialized.borrow_mut();
        let info = bm.as_mut().unwrap();
        render_component(self.clone(), info.props.clone());
    }
}

pub(crate) trait ComponentStore {
    fn render(&self);
}

struct InitializedComponentInfo<Ret, Props>
where
    Props: ComponentProps,
    Ret: ComponentReturn,
{
    func: ComponentFunc<Ret, Props>,
    props: Props,
    parent_node: Node,
    my_hole: Option<Ret::HoleNode>,
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

fn render_component<Ret, Props>(this: SharedPart<ComponentStoreInstance<Ret, Props>>, props: Props)
where
    Props: ComponentProps,
    Ret: ComponentReturn,
{
    let info = this.initialized.borrow_mut();
    let info = info.as_mut().unwrap();
    let stores = this.map(|ins| &ins.stores);
    let InitializedComponentInfo {
        func,
        my_hole,
        parent_node,
        ..
    } = info;

    let cc = ComponentBuilder {
        hook_builder: HookBuilder {
            untyped_stores: stores.upcast(),
            current_component: this,
        },
        parent_node: parent_node.clone(),
    };
    let hole = func(cc, props).get_node();

    *my_hole = Some(hole);
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
    /** Render the given component with the given prop.

    This consumes the `consecuit` or `cc` object, and returns a new one.

    This is equivalent to a tag in the [`cc_tree!`][consecuit_macros::cc_tree] macro.

    For example:
    ```
    cc_tree!(
        <div />
        <footer {html_props().class_name("hi")} />
    )
    ```

    is equivalent to

    ```
    cc.comp(div, Default::default())
        .comp(footer, html_props().class_name("hi"))
    ```
    */
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

        let last_node = match container_store.initialized.borrow_mut().deref_mut() {
            Some(info) => {
                if component_props != info.props {
                    render_component(container_store, component_props.clone());
                    info.props = component_props;
                }
                info.my_hole.clone().unwrap()
            }
            opt_none => {
                *opt_none = Some(InitializedComponentInfo {
                    func: component_func,
                    props: component_props.clone(),
                    my_hole: None,
                    parent_node: parent_node.clone(),
                });
                let info = opt_none.as_mut().unwrap();
                render_component(container_store, component_props);
                info.my_hole.clone().unwrap()
            }
        };
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
    /** Descend into the hole of the last component with the given closure.

    This consumes the `consecuit` or `cc` object, and returns a new one.

    Use this to nest components. For example:

    ```
    cc.comp(table, html_props())
        .child(|cc| {
            cc.comp(tr, html_props())
            .child(|cc| {
                cc.comp(td, html_props())
                .child(|cc| {
                    cc.comp(text_node, "hello")
                })
            })
        })
    ```

    The [`cc_tree!`][consecuit_macros::cc_tree] macro equivalent for the above code is:

    ```
    cc_tree!(
        <table {html_props()}>
            <tr {html_props()}>
                <td {html_props()}>
                    "hello"
                </td>
            </tr>
        </table>
    )
    ```

    Some restrictions:

    * This only work on components that returns `impl ContainerReturn`.
    * You can only use this once per component. `cc.comp(...).child(...).child(...)` is not valid.
    * This is mutaully exclusive with `.hole_here()`. `cc.comp(...).hole_here().child(...)` is not valid.

     */
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
