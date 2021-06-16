use crate::{holes::{HoleCons, HoleConsEnd, HolesList, HolesListNotEmpty}, hook::{HookBuilder, HookStores, HookReturn, Rerun, Rerunner}, stores::{StoreCons, StoreConsEnd, StoresList}, unmounted_lock::UnmountedLock};
use std::{cell::RefCell, marker::PhantomData, mem::transmute, ops::DerefMut};
use web_sys::{window, Node};

struct PanicRootRerunner();
impl Rerun for PanicRootRerunner {
    fn rerun(&'static self) {
        panic!("cannot rerun parent of the root component")
    }
}
static PANIC_RERUNNER: PanicRootRerunner = PanicRootRerunner();

pub fn mount<Ret, Func>(function: Func)
where
    Ret: ComponentReturn + 'static,
    Func: 'static + Fn(ComponentBuilder, ()) -> Ret,
{
    fn mount_inner<Ret, Func>(function: Func)
    where
        Ret: ComponentReturn + 'static,
        Func: 'static + FnOnce(ComponentBuilder, ()) -> Ret,
    {
        let s: Ret::StoresList = StoresList::create();
        let s = Box::new(s);
        let s: &Ret::StoresList = Box::leak(s);
        let untyped_stores: &'static () =
            unsafe { transmute::<&'_ Ret::StoresList, &'static ()>(s) };
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

pub trait ComponentReturn {
    type StoresList: StoresList;
    type HolesList: HolesList;
    fn get_holes(self) -> Self::HolesList;
}

type EmptyComponentStores<Entire> = ComponentStores<StoreConsEnd, Entire>;

impl<UsedStores, LastHoles, RetHoles> ComponentReturn for ComponentStoresWithNode<StoreConsEnd, UsedStores, LastHoles, RetHoles>
where
    UsedStores: StoresList,
    LastHoles: HolesList,
    RetHoles: HolesList
{
    type StoresList = UsedStores;
    type HolesList = RetHoles;
    fn get_holes(self) -> Self::HolesList {
        self.ret_holes
    }
}

pub struct ComponentStoresWithNode<CurrentStores: StoresList, EntireStores: StoresList, LastHoles: HolesList, RetHoles: HolesList> {
    pub(crate) hook_stores: HookStores<CurrentStores, EntireStores>,
    pub(crate) parent_node: Node,
    pub(crate) last_holes: LastHoles,
    pub(crate) ret_holes: RetHoles
}

fn run_component<Func, Props, Ret>(
    store: &'static Ret::StoresList,
    lock: UnmountedLock,
    rerun: Rerunner,
    component_func: &Func,
    props: Props,
    on_node: Node,
) -> Ret::HolesList
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
    let holes = component_func(reia, props).get_holes();
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
    Ret: ComponentReturn + 'static,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
{
    stores: Ret::StoresList,
    initialized: RefCell<Option<InitializedComponentInfo<Func, Ret, Props>>>,
}

struct InitializedComponentInfo<Func, Ret, Props>
where
    Props: 'static,
    Ret: ComponentReturn + 'static,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
{
    func: Func,
    props: Props,
    parent_node: Node,
    my_holes: Option<Ret::HolesList>,
    lock: UnmountedLock,
}

impl<Func, Ret, Props> Default for ComponentContainer<Func, Ret, Props>
where
    Props: 'static,
    Ret: ComponentReturn + 'static,
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
    Ret: ComponentReturn + 'static,
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
    Ret: ComponentReturn + 'static,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
    Props: PartialEq + Clone,
{
    pub fn node(
        self, component_func: Func, component_props: Props
    ) -> ComponentStoresWithNode<RestStores, EntireStores, Ret::HolesList, HoleConsEnd> {
        let ComponentStores { hook_stores, parent_node } = self;
        let with_node = ComponentStoresWithNode {
            hook_stores,
            parent_node,
            ret_holes: HoleConsEnd,
            last_holes: HoleConsEnd
        };
        with_node.node(component_func, component_props)
    }
}


impl<RestStores, EntireStores, Func, Ret, Props, LastHoles, RetHoles>
    ComponentStoresWithNode<StoreCons<ComponentContainer<Func, Ret, Props>, RestStores>, EntireStores, LastHoles, RetHoles>
where
    RestStores: StoresList,
    EntireStores: StoresList,
    LastHoles: HolesList,
    RetHoles: HolesList,
    Ret: ComponentReturn + 'static,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
    Props: PartialEq + Clone,
{
    pub fn node(
        self,
        component_func: Func,
        component_props: Props,
    ) -> ComponentStoresWithNode<RestStores, EntireStores, Ret::HolesList, RetHoles> {
        let ComponentStoresWithNode {
            hook_stores,
            parent_node,
            ret_holes,
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
        let last_holes = container_store
            .initialized
            .borrow()
            .as_ref()
            .unwrap()
            .my_holes
            .clone()
            .unwrap();
        ComponentStoresWithNode {
            hook_stores: rests,
            parent_node,
            last_holes,
            ret_holes
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
    ) -> ComponentStoresWithNode<StoreConsEnd, EntireStores, HoleConsEnd, HoleCons<HoleConsEnd>> {
        let ComponentStores {
            hook_stores,
            parent_node,
        } = self;
        ComponentStoresWithNode {
            hook_stores,
            parent_node,
            ret_holes: HoleCons (node, HoleConsEnd),
            last_holes: HoleConsEnd
        }
    }
    pub(crate) fn bare_leaf_node(
        self
    ) -> ComponentStoresWithNode<StoreConsEnd, EntireStores, HoleConsEnd, HoleConsEnd> {
        let ComponentStores {
            hook_stores,
            parent_node,
        } = self;
        ComponentStoresWithNode {
            hook_stores,
            parent_node,
            last_holes: HoleConsEnd,
            ret_holes: HoleConsEnd
        }
    }
}

impl<RestStores, EntireStores, Func, Ret, Props, OldLastHoles, RetHoles>
    ComponentStoresWithNode<
        StoreCons<ComponentContainer<Func, Ret, Props>, RestStores>,
        EntireStores,
        OldLastHoles,
        RetHoles
    >
where
    RestStores: StoresList,
    EntireStores: StoresList,
    OldLastHoles: HolesList,
    RetHoles: HolesList,
    Ret: ComponentReturn + 'static,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
    Props: PartialEq + Clone,
{
    pub fn sibling(
        self,
        component_func: Func,
        props: Props,
    ) -> ComponentStoresWithNode<RestStores, EntireStores, Ret::HolesList, RetHoles> {
        self.node(component_func, props)
    }
}

impl<ThisStore, RestStores, EntireStores, RestLastHoles, OldRetHoles>
    ComponentStoresWithNode<StoreCons<ThisStore, RestStores>, EntireStores, RestLastHoles, OldRetHoles>
where
    ThisStore: StoresList,
    RestStores: StoresList,
    EntireStores: StoresList,
    RestLastHoles: HolesList,
    OldRetHoles: HolesList
{
    pub fn child<Builder, ChildLastHoles, NewRetHoles>(
        self,
        builder: Builder,
    ) -> ComponentStoresWithNode<RestStores, EntireStores, RestLastHoles::Tail, NewRetHoles>
    where
        ChildLastHoles: HolesList,
        NewRetHoles: HolesList,
        Builder: Fn(
            ComponentStoresWithNode<ThisStore, ThisStore, HoleConsEnd, OldRetHoles>,
        ) -> ComponentStoresWithNode<StoreConsEnd, ThisStore, ChildLastHoles, NewRetHoles>,
    {
        let ComponentStoresWithNode {
            hook_stores,
            parent_node,
            last_holes,
            ret_holes
        } = self;
        let (hole, rest_holes) = last_holes.split_head();
        let (rest_stores, store) = hook_stores.use_one_store();
        let comp_stores = ComponentStoresWithNode {
            hook_stores: HookStores {
                current: store,
                entire: PhantomData,
                lock: rest_stores.lock.clone(),
                rerun: rest_stores.rerun.clone(),
            },
            parent_node: hole,
            last_holes: HoleConsEnd,
            ret_holes,
        };
        let built = builder(comp_stores);
        ComponentStoresWithNode {
            hook_stores: rest_stores,
            parent_node,
            last_holes: rest_holes,
            ret_holes: built.ret_holes
        }
    }
}

impl<Stores, EntireStores, OldLastHoles, OldRetHoles> ComponentStoresWithNode<Stores, EntireStores, OldLastHoles, OldRetHoles>
where
    Stores: StoresList,
    EntireStores: StoresList,
    OldLastHoles: HolesList,
    OldRetHoles: HolesList
{
    pub fn hole_here(self) -> ComponentStoresWithNode<Stores, EntireStores, OldLastHoles::Tail, HoleCons<OldRetHoles>> {
        let ComponentStoresWithNode { hook_stores, parent_node, last_holes, ret_holes } = self;
        let (old_last_head, new_last) = last_holes.split_head();
        let new_ret = ret_holes.append(old_last_head);
        ComponentStoresWithNode {
            hook_stores,
            parent_node,
            last_holes: new_last,
            ret_holes: new_ret
        }
    }
}