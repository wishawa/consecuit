use crate::{
    hook::{HookBuilder, HookReturn, HookStores, Rerun, Rerunner},
    stores::{StoreCons, StoreConsEnd, StoresList},
    unmounted_lock::UnmountedLock,
};
use std::{cell::RefCell, marker::PhantomData, mem::transmute, ops::DerefMut};
use web_sys::{window, Node};

struct PanicRootRerunner();
impl Rerun for PanicRootRerunner {
    fn rerun(&'static self) {
        unreachable!("impossible for parent of the root component to rerun")
    }
}
static PANIC_RERUNNER: PanicRootRerunner = PanicRootRerunner();

pub fn mount<Ret, Func>(function: Func)
where
    Ret: ComponentReturn,
    Func: 'static + Fn(ComponentBuilder, ()) -> Ret,
{
    let stores: StoreCons<ComponentContainer<Func, Ret, ()>, Ret::StoresList> =
        StoresList::create();
    let stores: &_ = Box::leak(Box::new(stores));
    let parent_node: Node = window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("reia-app-root")
        .unwrap()
        .into();
    let lock = UnmountedLock::new_mounted();
    type StillFullComponentStore<T> = ComponentStores<T, T>;
    let component_store: StillFullComponentStore<_> = ComponentStores {
        hook_stores: HookStores {
            current: stores,
            entire: PhantomData,
            lock: lock.clone(),
            rerun: Rerunner(&PANIC_RERUNNER),
        },
        parent_node,
    };
    component_store.node(function, ());
}

pub struct ComponentBuilder {
    pub(crate) hook_builder: HookBuilder,
    pub(crate) parent_node: Node,
}

pub struct ComponentStores<CurrentStores: StoresList, EntireStores: StoresList> {
    pub(crate) hook_stores: HookStores<CurrentStores, EntireStores>,
    pub(crate) parent_node: Node,
}

impl ComponentBuilder {
    pub fn init<T: StoresList>(self) -> ComponentStores<T, T> {
        let Self {
            hook_builder,
            parent_node,
        } = self;
        ComponentStores {
            hook_stores: hook_builder.init::<T>(),
            parent_node,
        }
    }
}

pub trait ComponentReturn: 'static {
    type StoresList: StoresList;
    type HoleNode: MaybeHoleNode;
    fn get_node(self) -> Self::HoleNode;
}

pub trait ContainerReturn: ComponentReturn<HoleNode = Node> {}
impl<T: ComponentReturn<HoleNode = Node>> ContainerReturn for T {}

impl<Stores: StoresList, LastNode: MaybeHoleNode, HoleNode: MaybeHoleNode> ComponentReturn
    for NodeComponentStores<StoreConsEnd, Stores, LastNode, HoleNode>
{
    type StoresList = Stores;
    type HoleNode = HoleNode;
    fn get_node(self) -> Self::HoleNode {
        self.ret_node
    }
}

pub trait MaybeHoleNode: 'static + Clone {}
impl MaybeHoleNode for Node {}
impl MaybeHoleNode for () {}
pub struct NodeComponentStores<
    CurrentStores: StoresList,
    EntireStores: StoresList,
    LastNode: MaybeHoleNode,
    ReturnNode: MaybeHoleNode,
> {
    pub(crate) hook_stores: HookStores<CurrentStores, EntireStores>,
    pub(crate) parent_node: Node,
    pub(crate) last_node: LastNode,
    pub(crate) ret_node: ReturnNode,
}

fn run_component<Func, Props, Ret>(
    store: &'static Ret::StoresList,
    lock: UnmountedLock,
    rerun: Rerunner,
    component_func: &Func,
    props: Props,
    on_node: Node,
) -> Ret::HoleNode
where
    Ret: ComponentReturn,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
{
    let untyped_stores = unsafe { transmute::<&'static Ret::StoresList, &'static ()>(store) };
    let reia = ComponentBuilder {
        hook_builder: HookBuilder {
            untyped_stores,
            lock,
            rerun,
        },
        parent_node: on_node,
    };
    let holes = component_func(reia, props).get_node();
    holes
}

impl<ThisStore, RestStores, EntireStores>
    ComponentStores<StoreCons<ThisStore, RestStores>, EntireStores>
where
    ThisStore: StoresList,
    RestStores: StoresList,
    EntireStores: StoresList,
{
    pub fn hook<Func, Arg, Out, Ret>(
        self,
        hook_func: Func,
        hook_arg: Arg,
    ) -> (ComponentStores<RestStores, EntireStores>, Out)
    where
        Ret: HookReturn<Out, StoresList = ThisStore>,
        Func: 'static + Fn(HookBuilder, Arg) -> Ret,
    {
        let ComponentStores {
            hook_stores,
            parent_node: node,
        } = self;
        let (hook_stores, out) = hook_stores.hook(hook_func, hook_arg);
        let comp_stores = ComponentStores {
            hook_stores,
            parent_node: node,
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
    stores: Ret::StoresList,
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
    parent_node: Node,
    my_holes: Option<Ret::HoleNode>,
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

impl<Func, Ret, Props> Rerun for ComponentContainer<Func, Ret, Props>
where
    Props: 'static + Clone,
    Ret: ComponentReturn,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
{
    fn rerun(self: &'static Self) {
        let mut ran_borrow = self.initialized.borrow_mut();
        let stores = &self.stores;
        let InitializedComponentInfo {
            func,
            props,
            my_holes,
            lock,
            parent_node,
        } = ran_borrow.deref_mut().as_mut().unwrap();
        let rerun = Rerunner(self);
        *my_holes = Some(run_component(
            stores,
            lock.clone(),
            rerun,
            func,
            props.clone(),
            parent_node.clone(),
        ));
    }
}

impl<RestStores, EntireStores, Func, Ret, Props>
    ComponentStores<StoreCons<ComponentContainer<Func, Ret, Props>, RestStores>, EntireStores>
where
    RestStores: StoresList,
    EntireStores: StoresList,
    Ret: ComponentReturn,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
    Props: PartialEq + Clone,
{
    pub fn node(
        self,
        component_func: Func,
        component_props: Props,
    ) -> NodeComponentStores<RestStores, EntireStores, Ret::HoleNode, ()> {
        let ComponentStores {
            hook_stores,
            parent_node,
        } = self;
        let with_node: NodeComponentStores<_, _, (), ()> = NodeComponentStores {
            hook_stores,
            parent_node,
            last_node: (),
            ret_node: (),
        };
        with_node.node(component_func, component_props)
    }
}

impl<RestStores, EntireStores, Func, Ret, Props, LastNode, CompHole>
    NodeComponentStores<
        StoreCons<ComponentContainer<Func, Ret, Props>, RestStores>,
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
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
    Props: PartialEq + Clone,
{
    pub fn node(
        self,
        component_func: Func,
        component_props: Props,
    ) -> NodeComponentStores<RestStores, EntireStores, Ret::HoleNode, CompHole> {
        let NodeComponentStores {
            hook_stores,
            parent_node,
            ret_node,
            ..
        } = self;
        let (rests, container_store) = hook_stores.use_one_store();
        let should_rerun = {
            let mut ran_borrow = container_store.initialized.borrow_mut();
            match ran_borrow.deref_mut() {
                Some(ran_info) => {
                    let props = &mut ran_info.props;
                    if component_props != *props {
                        *props = component_props.clone();
                        true
                    } else {
                        false
                    }
                }
                opt => {
                    *opt = Some(InitializedComponentInfo {
                        func: component_func,
                        props: component_props,
                        my_holes: None,
                        lock: rests.lock.clone(),
                        parent_node: parent_node.clone(),
                    });
                    true
                }
            }
        };
        let rerunner = Rerunner(container_store);
        if should_rerun {
            rerunner.rerun();
        }
        let last_node = container_store
            .initialized
            .borrow()
            .as_ref()
            .unwrap()
            .my_holes
            .clone()
            .unwrap();
        NodeComponentStores {
            hook_stores: rests,
            parent_node,
            ret_node,
            last_node,
        }
    }
}

impl<EntireStores> ComponentStores<StoreConsEnd, EntireStores>
where
    EntireStores: StoresList,
{
    pub(crate) fn bare_container_node(
        self,
        node: Node,
    ) -> NodeComponentStores<StoreConsEnd, EntireStores, (), Node> {
        let ComponentStores {
            hook_stores,
            parent_node,
        } = self;
        NodeComponentStores {
            hook_stores,
            parent_node,
            last_node: (),
            ret_node: node,
        }
    }
    pub(crate) fn bare_leaf_node(self) -> NodeComponentStores<StoreConsEnd, EntireStores, (), ()> {
        let ComponentStores {
            hook_stores,
            parent_node,
        } = self;
        NodeComponentStores {
            hook_stores,
            parent_node,
            last_node: (),
            ret_node: (),
        }
    }
}

impl<RestStores, EntireStores, Func, Ret, Props, LastNode, CompHole>
    NodeComponentStores<
        StoreCons<ComponentContainer<Func, Ret, Props>, RestStores>,
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
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
    Props: PartialEq + Clone,
{
    pub fn sibling(
        self,
        component_func: Func,
        props: Props,
    ) -> NodeComponentStores<RestStores, EntireStores, Ret::HoleNode, CompHole> {
        self.node(component_func, props)
    }
}

impl<ThisStore, RestStores, EntireStores, CompHole>
    NodeComponentStores<StoreCons<ThisStore, RestStores>, EntireStores, Node, CompHole>
where
    ThisStore: StoresList,
    RestStores: StoresList,
    EntireStores: StoresList,
    CompHole: MaybeHoleNode,
{
    pub fn child<Builder, ChildLastNode, ChildHole>(
        self,
        builder: Builder,
    ) -> NodeComponentStores<RestStores, EntireStores, (), ChildHole>
    where
        ChildHole: MaybeHoleNode,
        ChildLastNode: MaybeHoleNode,
        Builder: Fn(
            NodeComponentStores<ThisStore, ThisStore, (), CompHole>,
        )
            -> NodeComponentStores<StoreConsEnd, ThisStore, ChildLastNode, ChildHole>,
    {
        let NodeComponentStores {
            hook_stores,
            parent_node,
            ret_node,
            last_node,
        } = self;
        let (rest_stores, store) = hook_stores.use_one_store();
        let comp_stores = NodeComponentStores {
            hook_stores: HookStores {
                current: store,
                entire: PhantomData,
                lock: rest_stores.lock.clone(),
                rerun: rest_stores.rerun.clone(),
            },
            parent_node: last_node,
            ret_node,
            last_node: (),
        };
        let built = builder(comp_stores);
        NodeComponentStores {
            hook_stores: rest_stores,
            parent_node,
            last_node: (),
            ret_node: built.ret_node,
        }
    }
}

impl<Stores, EntireStores> NodeComponentStores<Stores, EntireStores, Node, ()>
where
    Stores: StoresList,
    EntireStores: StoresList,
{
    pub fn hole_here(self) -> NodeComponentStores<Stores, EntireStores, (), Node> {
        let NodeComponentStores {
            hook_stores,
            parent_node,
            last_node,
            ret_node,
        } = self;
        NodeComponentStores {
            hook_stores,
            parent_node,
            last_node: ret_node,
            ret_node: last_node,
        }
    }
}
