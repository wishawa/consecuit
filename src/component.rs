use crate::{holes::{HoleNodeCons, HoleNodeConsEnd, HoleNodesList}, hook::{HookBuilder, HookStores, HookValue, Rerun, Rerunner}, stores::{StoreCons, StoreConsEnd, StoresList}, unmounted_lock::UnmountedLock};
use std::{cell::RefCell, marker::PhantomData, mem::transmute, ops::DerefMut};
use web_sys::{window, Node};

struct PanicRootRerunner();
impl Rerun for PanicRootRerunner {
    fn rerun(&'static self) {
        panic!("cannot rerun parent of the root component")
    }
}
static PANIC_RERUNNER: PanicRootRerunner = PanicRootRerunner();

pub fn mount<Stores, Ret, Func>(function: Func)
where
    Stores: StoresList,
    Ret: ComponentValue<StoresList = Stores> + 'static,
    Func: 'static + Fn(ComponentBuilder, ()) -> Ret,
{
    fn mount_inner<Stores, Ret, Func>(function: Func)
    where
        Stores: StoresList,
        Ret: ComponentValue<StoresList = Stores> + 'static,
        Func: 'static + FnOnce(ComponentBuilder, ()) -> Ret,
    {
        let s: Stores = Stores::create();
        let s = Box::new(s);
        let s: &Stores = Box::leak(s);
        let untyped_stores: &'static () = unsafe { transmute::<&'_ Stores, &'static ()>(s) };
        let lock = UnmountedLock::new_mounted();
        // let lock_1 = lock.clone();

        let root_rerunner = Rerunner(&PANIC_RERUNNER);
        let reia = ComponentBuilder {
            hook_builder: HookBuilder {
                untyped_stores: untyped_stores,
                lock,
                rerun: root_rerunner,
            },
            parent_node: window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("reia-app-root")
                .unwrap()
                .into(),
        };
        function(reia, ());
        // Do something
        // lock_1.unmount();
    }

    let wrapped = |reia: ComponentBuilder, _: ()| {
        let reia = reia.init();
        reia.node(function, ())
    };
    mount_inner(wrapped);
}

pub struct ComponentBuilder {
    pub(crate) hook_builder: HookBuilder,
    pub(crate) parent_node: Node,
}

pub struct ComponentStores<CurrentStores: StoresList, EntireStores: StoresList> {
    hook_stores: HookStores<CurrentStores, EntireStores>,
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

pub trait ComponentValue {
    type StoresList: StoresList;
    type HolesList: HoleNodesList;
    fn get_holes(self) -> Self::HolesList;
}

type EmptyComponentStores<Entire> = ComponentStores<StoreConsEnd, Entire>;

impl<UsedStores, Holes> ComponentValue for ComponentStoresWithNode<StoreConsEnd, UsedStores, Holes>
where
    UsedStores: StoresList, Holes: HoleNodesList
{
    type StoresList = UsedStores;
    type HolesList = Holes;
    fn get_holes(self) -> Self::HolesList {
        self.hole_nodes
    }
}

pub struct ComponentStoresWithNode<CurrentStores: StoresList, EntireStores: StoresList, Holes: HoleNodesList> {
    pub(crate) hook_stores: HookStores<CurrentStores, EntireStores>,
    pub(crate) parent_node: Node,
    pub(crate) hole_nodes: Holes
}

fn run_component<Stores, Func, Props, Ret, Holes>(
    store: &'static Stores,
    lock: UnmountedLock,
    rerun: Rerunner,
    component_func: &Func,
    props: Props,
    on_node: Node,
) -> Holes
where
    Stores: StoresList,
    Holes: HoleNodesList,
    Ret: ComponentValue<StoresList = Stores, HolesList = Holes>,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
{
    let untyped_stores = unsafe { transmute::<&'static Stores, &'static ()>(store) };
    let reia = ComponentBuilder {
        hook_builder: HookBuilder {
            untyped_stores,
            lock,
            rerun,
        },
        parent_node: on_node,
    };
    component_func(reia, props).get_holes()
}

impl<ThisStores, RestStores, EntireStores>
    ComponentStores<StoreCons<ThisStores, RestStores>, EntireStores>
where
    ThisStores: StoresList,
    RestStores: StoresList,
    EntireStores: StoresList,
{
    pub fn hook<Func, Arg, Out, Ret>(
        self,
        hook_func: Func,
        hook_arg: Arg,
    ) -> (ComponentStores<RestStores, EntireStores>, Out)
    where
        Ret: HookValue<Out, StoresList = ThisStores>,
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

pub struct ComponentContainer<Stores, Func, Ret, Props, Holes>
where
    Stores: StoresList,
    Props: 'static,
    Holes: HoleNodesList,
    Ret: ComponentValue<StoresList = Stores, HolesList = Holes> + 'static,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
{
    stores: Stores,
    initialized: RefCell<Option<InitializedComponentInfo<Stores, Func, Ret, Props, Holes>>>,
}

struct InitializedComponentInfo<Stores, Func, Ret, Props, Holes>
where
    Stores: StoresList,
    Props: 'static,
    Holes: HoleNodesList,
    Ret: ComponentValue<StoresList = Stores, HolesList = Holes> + 'static,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
{
    func: Func,
    props: Props,
    parent_node: Node,
    ret_holes: Option<Holes>,
    lock: UnmountedLock,
}

impl<Stores, Func, Ret, Props, Holes> Default for ComponentContainer<Stores, Func, Ret, Props, Holes>
where
    Stores: StoresList,
    Props: 'static,
    Holes: HoleNodesList,
    Ret: ComponentValue<StoresList = Stores, HolesList = Holes> + 'static,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
{
    fn default() -> Self {
        Self {
            stores: Stores::create(),
            initialized: RefCell::new(None),
        }
    }
}

impl<Stores, Func, Ret, Props, Holes> Rerun for ComponentContainer<Stores, Func, Ret, Props, Holes>
where
    Stores: StoresList,
    Props: 'static + Clone,
    Holes: HoleNodesList,
    Ret: ComponentValue<StoresList = Stores, HolesList = Holes> + 'static,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
{
    fn rerun(self: &'static Self) {
        let mut ran_borrow = self.initialized.borrow_mut();
        let stores = &self.stores;
        let InitializedComponentInfo {
            func,
            props,
            ret_holes,
            lock,
            parent_node,
        } = ran_borrow.deref_mut().as_mut().unwrap();
        let rerun = Rerunner(self);
        *ret_holes = Some(run_component(
            stores,
            lock.clone(),
            rerun,
            func,
            props.clone(),
            parent_node.clone(),
        ));
    }
}

impl<ThisStores, RestStores, EntireStores, Func, Ret, Props, Holes>
    ComponentStores<
        StoreCons<ComponentContainer<ThisStores, Func, Ret, Props, Holes>, RestStores>,
        EntireStores,
    >
where
    ThisStores: StoresList,
    RestStores: StoresList,
    EntireStores: StoresList,
    Holes: HoleNodesList,
    Ret: ComponentValue<StoresList = ThisStores, HolesList = Holes> + 'static,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
    Props: PartialEq + Clone,
{
    pub fn node(
        self,
        component_func: Func,
        component_props: Props,
    ) -> ComponentStoresWithNode<RestStores, EntireStores, Holes> {
        let ComponentStores {
            hook_stores,
            parent_node,
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
                        ret_holes: None,
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
        let hole_nodes = container_store
            .initialized
            .borrow()
            .as_ref()
            .unwrap()
            .ret_holes
            .clone()
            .unwrap();
        ComponentStoresWithNode {
            hook_stores: rests,
            parent_node,
            hole_nodes,
        }
    }
}

impl<EntireStores> EmptyComponentStores<EntireStores>
where
    EntireStores: StoresList,
{
    pub(crate) fn bare_container_node(
        self,
        node: Node,
    ) -> ComponentStoresWithNode<StoreConsEnd, EntireStores, HoleNodeCons<HoleNodeConsEnd>> {
        let ComponentStores {
            hook_stores,
            parent_node,
        } = self;
        ComponentStoresWithNode {
            hook_stores,
            parent_node,
            hole_nodes: HoleNodeCons (node, HoleNodeConsEnd ())
        }
    }
    pub(crate) fn bare_leaf_node(
        self
    ) -> ComponentStoresWithNode<StoreConsEnd, EntireStores, HoleNodeConsEnd> {
        let ComponentStores {
            hook_stores,
            parent_node,
        } = self;
        ComponentStoresWithNode {
            hook_stores,
            parent_node,
            hole_nodes: HoleNodeConsEnd ()
        }
    }
}

impl<ThisStores, RestStores, EntireStores, Func, Ret, Props, Holes>
    ComponentStoresWithNode<
        StoreCons<ComponentContainer<ThisStores, Func, Ret, Props, Holes>, RestStores>,
        EntireStores,
        HoleNodeConsEnd
    >
where
    ThisStores: StoresList,
    RestStores: StoresList,
    EntireStores: StoresList,
    Holes: HoleNodesList,
    Ret: ComponentValue<StoresList = ThisStores, HolesList = Holes> + 'static,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
    Props: PartialEq + Clone,
{
    pub fn sibling(
        self,
        component_func: Func,
        props: Props,
    ) -> ComponentStoresWithNode<RestStores, EntireStores, Holes> {
        let ComponentStoresWithNode {
            hook_stores,
            parent_node,
            ..
        } = self;
        let comp_stores = ComponentStores {
            hook_stores,
            parent_node,
        };
        comp_stores.node(component_func, props)
    }
}

impl<ThisStores, RestStores, EntireStores, RestHoles>
    ComponentStoresWithNode<StoreCons<ThisStores, RestStores>, EntireStores, HoleNodeCons<RestHoles>>
where
    ThisStores: StoresList,
    RestStores: StoresList,
    EntireStores: StoresList,
    RestHoles: HoleNodesList
{
    pub fn child<Builder>(
        self,
        builder: Builder,
    ) -> ComponentStoresWithNode<RestStores, EntireStores, RestHoles>
    where
        Builder: Fn(
            ComponentStores<ThisStores, ThisStores>,
        ) -> ComponentStoresWithNode<StoreConsEnd, ThisStores, HoleNodeConsEnd>,
    {
        let ComponentStoresWithNode {
            hook_stores,
            parent_node,
            hole_nodes
        } = self;
        let (rest_stores, store) = hook_stores.use_one_store();
        let comp_stores = ComponentStores {
            hook_stores: HookStores {
                current: store,
                entire: PhantomData,
                lock: rest_stores.lock.clone(),
                rerun: rest_stores.rerun.clone(),
            },
            parent_node: hole_nodes.0,
        };
        builder(comp_stores);
        ComponentStoresWithNode {
            hook_stores: rest_stores,
            parent_node,
            hole_nodes: hole_nodes.1,
        }
    }
}
