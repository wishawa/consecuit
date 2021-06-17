use crate::{executor::{Renderable, RerenderTask}, hook::{HookBuilder, HookReturn, HookStores}, props::ComponentProps, stores::{StoreCons, StoreConsEnd, StoresList}, unmounted_lock::UnmountedLock};
use std::{borrow::Borrow, cell::RefCell, marker::PhantomData, mem::transmute, ops::DerefMut};
use web_sys::{window, Element};

pub(crate) trait FuncComp {
    type Ret;
}

impl<Func, Ret> FuncComp for Func
where
    Ret: ComponentReturn,
    Func: 'static + Fn(ComponentBuilder, Ret::Props) -> Ret
{
    type Ret = Ret;
}

pub fn mount<Func, Ret>(function: Func)
where
    Ret: ComponentReturn,
    Func: 'static + Fn(ComponentBuilder, ()) -> Ret,
{
    let document = window().unwrap().document().unwrap();

    let app_root: Element = document.get_element_by_id("reia-app-root").unwrap();
    let parent_node: Element = document.create_element("div").unwrap();

    let root_tree = create_subtree(parent_node.clone());
    root_tree.run(function, ());

    app_root.append_child(&parent_node).unwrap();

    Box::leak(Box::new(root_tree));
}

type TreeStores<Func, Ret, Props> =
    StoreCons<ComponentContainer<Func, Ret, Props>, <Ret as ComponentReturn>::Stores>;

pub(crate) struct ReiaSubtree<Func, Ret, Props>
where
    Ret: ComponentReturn,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
    Props: ComponentProps,
{
    stores: Box<TreeStores<Func, Ret, Props>>,
    lock: UnmountedLock,
    container: Element,
}

impl<Func, Ret, Props> Drop for ReiaSubtree<Func, Ret, Props>
where
    Ret: ComponentReturn,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
    Props: ComponentProps,
{
    fn drop(&mut self) {
        self.lock.unmount();
        self.container.remove();
    }
}

impl<Func, Ret, Props> ReiaSubtree<Func, Ret, Props>
where
    Ret: ComponentReturn,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
    Props: ComponentProps,
{
    pub(crate) fn run(&self, function: Func, props: Props) {
        struct PanicRootRerunner();
        impl Renderable for PanicRootRerunner {
            fn render(&'static self) {
                unreachable!("this dummy is never directly called")
            }
        }
        static PANIC_RERUNNER: PanicRootRerunner = PanicRootRerunner();

        let stores_borrow = unsafe {
            // This unsafe is really unsafe.
            // The stores list is NOT really &'static.
            // Always check the UnmountedLock before accessing it.
            transmute::<&'_ TreeStores<Func, Ret, Props>, &'static TreeStores<Func, Ret, Props>>(
                self.stores.borrow(),
            )
        };

        type StillFullNodeComponentStore<T, P> = ComponentStores<T, T, NoHoleNode, NoHoleNode, P>;
        let component_store: StillFullNodeComponentStore<_, Props> = ComponentStores {
            hook_stores: HookStores {
                current: stores_borrow,
                entire: PhantomData,
                lock: self.lock.clone(),
                rerender_parent: RerenderTask {
                    obj: &PANIC_RERUNNER,
                    lock: self.lock.clone(),
                },
            },
            parent_node: self.container.clone(),
            last_node: NoHoleNode,
            ret_node: NoHoleNode,
            props_phantom: PhantomData
        };
        component_store.node(function, props);
    }
}

pub(crate) fn create_subtree<Func, Ret, Props>(container: Element) -> ReiaSubtree<Func, Ret, Props>
where
    Ret: ComponentReturn,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
    Props: ComponentProps,
{
    let stores: TreeStores<Func, Ret, Props> = StoresList::create();
    let stores = Box::new(stores);
    let lock = UnmountedLock::new_mounted();
    ReiaSubtree {
        stores,
        lock,
        container,
    }
}

pub struct ComponentBuilder {
    pub(crate) hook_builder: HookBuilder,
    pub(crate) parent_node: Element,
}

impl ComponentBuilder {
    pub fn init<T: StoresList, P: ComponentProps>(self) -> ComponentStores<T, T, NoHoleNode, NoHoleNode, P> {
        let Self {
            hook_builder,
            parent_node,
        } = self;
        ComponentStores {
            hook_stores: hook_builder.init::<T>(),
            parent_node,
            last_node: NoHoleNode,
            ret_node: NoHoleNode,
            props_phantom: PhantomData
        }
    }
}

pub trait ComponentReturn: 'static {
    type Stores: StoresList;
    type Props: ComponentProps;
    type HoleNode: MaybeHoleNode;
    fn get_node(self) -> Self::HoleNode;
}

pub trait ContainerReturn: ComponentReturn<HoleNode = Element> {}
impl<T: ComponentReturn<HoleNode = Element>> ContainerReturn for T {}

impl<Stores, LastNode, HoleNode, Props> ComponentReturn
    for ComponentStores<StoreConsEnd, Stores, LastNode, HoleNode, Props>
where Stores: StoresList, LastNode: MaybeHoleNode, HoleNode: MaybeHoleNode, Props: ComponentProps {
    type Stores = Stores;
    type HoleNode = HoleNode;
    type Props = Props;
    fn get_node(self) -> Self::HoleNode {
        self.ret_node
    }
}

pub trait MaybeHoleNode: 'static + Clone {}
impl MaybeHoleNode for Element {}
#[derive(Clone)]
pub struct NoHoleNode;
impl MaybeHoleNode for NoHoleNode {}
pub struct ComponentStores<
    CurrentStores: StoresList,
    EntireStores: StoresList,
    LastNode: MaybeHoleNode,
    ReturnNode: MaybeHoleNode,
    Props: ComponentProps
> {
    pub(crate) hook_stores: HookStores<CurrentStores, EntireStores>,
    pub(crate) parent_node: Element,
    pub(crate) last_node: LastNode,
    pub(crate) ret_node: ReturnNode,
    pub(crate) props_phantom: PhantomData<Props>
}

fn run_component<Func, Props, Ret>(
    store: &'static Ret::Stores,
    lock: UnmountedLock,
    rerender: RerenderTask,
    component_func: &Func,
    props: Props,
    on_node: Element,
) -> Ret::HoleNode
where
    Ret: ComponentReturn,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
{
    let untyped_stores = unsafe { transmute::<&'static Ret::Stores, &'static ()>(store) };
    let reia = ComponentBuilder {
        hook_builder: HookBuilder {
            untyped_stores,
            lock,
            rerender_parent: rerender,
        },
        parent_node: on_node,
    };
    let holes = component_func(reia, props).get_node();
    holes
}

impl<ThisStore, RestStores, EntireStores, LastNode, CompHole, Props>
    ComponentStores<StoreCons<ThisStore, RestStores>, EntireStores, LastNode, CompHole, Props>
where
    ThisStore: StoresList,
    RestStores: StoresList,
    EntireStores: StoresList,
    LastNode: MaybeHoleNode,
    CompHole: MaybeHoleNode,
    Props: ComponentProps
{
    pub fn hook<Func, Arg, Out, Ret>(
        self,
        hook_func: Func,
        hook_arg: Arg,
    ) -> (
        ComponentStores<RestStores, EntireStores, LastNode, CompHole, Props>,
        Out,
    )
    where
        Ret: HookReturn<Out, StoresList = ThisStore>,
        Func: 'static + Fn(HookBuilder, Arg) -> Ret,
    {
        let ComponentStores {
            hook_stores,
            parent_node: node,
            last_node,
            ret_node,
            props_phantom
        } = self;
        let (hook_stores, out) = hook_stores.hook(hook_func, hook_arg);
        let comp_stores = ComponentStores {
            hook_stores,
            parent_node: node,
            last_node,
            ret_node,
            props_phantom
        };
        (comp_stores, out)
    }
}

pub struct ComponentContainer<Func, Ret, Props>
where
    Props: 'static,
    Ret: ComponentReturn,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
{
    stores: Ret::Stores,
    initialized: RefCell<Option<InitializedComponentInfo<Func, Ret, Props>>>,
}

struct InitializedComponentInfo<Func, Ret, Props>
where
    Props: 'static,
    Ret: ComponentReturn,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
{
    func: Func,
    props: Props,
    parent_node: Element,
    my_hole: Option<Ret::HoleNode>,
    lock: UnmountedLock,
}

impl<Func, Ret, Props> Default for ComponentContainer<Func, Ret, Props>
where
    Props: 'static,
    Ret: ComponentReturn,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
{
    fn default() -> Self {
        Self {
            stores: Ret::StoresList::create(),
            initialized: RefCell::new(None),
        }
    }
}

impl<Func, Ret, Props> Renderable for ComponentContainer<Func, Ret, Props>
where
    Props: 'static + Clone,
    Ret: ComponentReturn,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
{
    fn render(self: &'static Self) {
        let mut ran_borrow = self.initialized.borrow_mut();
        let stores = &self.stores;
        let InitializedComponentInfo {
            func,
            props,
            my_hole,
            lock,
            parent_node,
        } = ran_borrow.deref_mut().as_mut().unwrap();
        let rerender = RerenderTask {
            obj: self,
            lock: lock.clone(),
        };
        *my_hole = Some(run_component(
            stores,
            lock.clone(),
            rerender,
            func,
            props.clone(),
            parent_node.clone(),
        ));
    }
}

impl<RestStores, EntireStores, Func, Ret, NodeProps, LastNode, CompHole, CompProps>
    ComponentStores<
        StoreCons<ComponentContainer<Func, Ret, NodeProps>, RestStores>,
        EntireStores,
        LastNode,
        CompHole,
        CompProps
    >
where
    RestStores: StoresList,
    EntireStores: StoresList,
    LastNode: MaybeHoleNode,
    CompHole: MaybeHoleNode,
    Ret: ComponentReturn,
    Func: 'static + Fn(ComponentBuilder, NodeProps) -> Ret,
    NodeProps: ComponentProps,
    CompProps: ComponentProps,
{
    pub fn node(
        self,
        component_func: Func,
        component_props: NodeProps,
    ) -> ComponentStores<RestStores, EntireStores, Ret::HoleNode, CompHole, CompProps> {
        let ComponentStores {
            hook_stores,
            parent_node,
            ret_node,
            props_phantom,
            ..
        } = self;
        let (rests, container_store) = hook_stores.use_one_store();
        let should_render = {
            let mut ran_borrow = container_store.initialized.borrow_mut();
            match ran_borrow.deref_mut() {
                Some(ran_info) => {
                    let props = &mut ran_info.props;
                    if component_props.props_eq(props) {
                        false
                    }
                    else {
                        *props = component_props.clone();
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
        ComponentStores {
            hook_stores: rests,
            parent_node,
            ret_node,
            last_node,
            props_phantom
        }
    }
}

impl<EntireStores, Props> ComponentStores<StoreConsEnd, EntireStores, NoHoleNode, NoHoleNode, Props>
where
    EntireStores: StoresList,
    Props: ComponentProps
{
    pub(crate) fn bare_container_node(
        self,
        node: Element,
    ) -> ComponentStores<StoreConsEnd, EntireStores, NoHoleNode, Element, Props> {
        let ComponentStores {
            hook_stores,
            parent_node,
            props_phantom,
            ..
        } = self;
        ComponentStores {
            hook_stores,
            parent_node,
            last_node: NoHoleNode,
            ret_node: node,
            props_phantom
        }
    }
    pub(crate) fn bare_leaf_node(
        self,
    ) -> ComponentStores<StoreConsEnd, EntireStores, NoHoleNode, NoHoleNode, Props> {
        let ComponentStores {
            hook_stores,
            parent_node,
            props_phantom,
            ..
        } = self;
        ComponentStores {
            hook_stores,
            parent_node,
            last_node: NoHoleNode,
            ret_node: NoHoleNode,
            props_phantom
        }
    }
}

impl<ThisStore, RestStores, EntireStores, CompHole, Props>
    ComponentStores<StoreCons<ThisStore, RestStores>, EntireStores, Element, CompHole, Props>
where
    ThisStore: StoresList,
    RestStores: StoresList,
    EntireStores: StoresList,
    CompHole: MaybeHoleNode,
    Props: ComponentProps
{
    pub fn child<Builder, ChildLastNode, ChildHole>(
        self,
        builder: Builder,
    ) -> ComponentStores<RestStores, EntireStores, NoHoleNode, ChildHole, Props>
    where
        ChildHole: MaybeHoleNode,
        ChildLastNode: MaybeHoleNode,
        Builder: FnOnce(
            ComponentStores<ThisStore, ThisStore, NoHoleNode, CompHole, ()>,
        )
            -> ComponentStores<StoreConsEnd, ThisStore, ChildLastNode, ChildHole, ()>,
    {
        let ComponentStores {
            hook_stores,
            parent_node,
            ret_node,
            last_node,
            props_phantom
        } = self;
        let (rest_stores, store) = hook_stores.use_one_store();
        let comp_stores = ComponentStores {
            hook_stores: HookStores {
                current: store,
                entire: PhantomData,
                lock: rest_stores.lock.clone(),
                rerender_parent: rest_stores.rerender_parent.clone(),
            },
            parent_node: last_node,
            ret_node,
            last_node: NoHoleNode,
            props_phantom: PhantomData
        };
        let built = builder(comp_stores);
        ComponentStores {
            hook_stores: rest_stores,
            parent_node,
            last_node: NoHoleNode,
            ret_node: built.ret_node,
            props_phantom
        }
    }
}

impl<Stores, EntireStores, Props> ComponentStores<Stores, EntireStores, Element, NoHoleNode, Props>
where
    Stores: StoresList,
    EntireStores: StoresList,
    Props: ComponentProps
{
    pub fn hole_here(self) -> ComponentStores<Stores, EntireStores, NoHoleNode, Element, Props> {
        let ComponentStores {
            hook_stores,
            parent_node,
            last_node,
            ret_node,
            props_phantom
        } = self;
        ComponentStores {
            hook_stores,
            parent_node,
            last_node: ret_node,
            ret_node: last_node,
            props_phantom
        }
    }
}
