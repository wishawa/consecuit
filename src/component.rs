use crate::{
    hook::{HookBuilder, HookStores, HookReturn, Rerun, Rerunner},
    stores::{StoreCons, StoreConsEnd, StoresList},
    unmounted_lock::UnmountedLock,
};
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
    fn get_node(self) -> Node;
}

type EmptyComponentStores<Entire> = ComponentStores<StoreConsEnd, Entire>;

impl<UsedStores> ComponentReturn for ComponentStoresWithNode<StoreConsEnd, UsedStores>
where
    UsedStores: StoresList,
{
    type StoresList = UsedStores;
    fn get_node(self) -> Node {
        self.last_node
    }
}

pub struct ComponentStoresWithNode<CurrentStores: StoresList, EntireStores: StoresList> {
    pub(crate) hook_stores: HookStores<CurrentStores, EntireStores>,
    pub(crate) parent_node: Node,
    pub(crate) last_node: Node,
}

fn run_component<Func, Props, Ret>(
    store: &'static Ret::StoresList,
    lock: UnmountedLock,
    rerun: Rerunner,
    component_func: &Func,
    props: Props,
    on_node: Node,
) -> Node
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
    let node = component_func(reia, props).get_node();
    node
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
    ret_node: Option<Node>,
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
            ret_node,
            lock,
            parent_node,
        } = ran_borrow.deref_mut().as_mut().unwrap();
        let rerun = Rerunner(self);
        *ret_node = Some(run_component(
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
        self,
        component_func: Func,
        component_props: Props,
    ) -> ComponentStoresWithNode<RestStores, EntireStores> {
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
                        ret_node: None,
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
        let node = container_store
            .initialized
            .borrow()
            .as_ref()
            .unwrap()
            .ret_node
            .clone()
            .unwrap();
        ComponentStoresWithNode {
            hook_stores: rests,
            parent_node,
            last_node: node,
        }
    }
}

impl<EntireStores> EmptyComponentStores<EntireStores>
where
    EntireStores: StoresList,
{
    pub(crate) fn bare_node(
        self,
        node: Node,
    ) -> ComponentStoresWithNode<StoreConsEnd, EntireStores> {
        let ComponentStores {
            hook_stores,
            parent_node,
        } = self;
        ComponentStoresWithNode {
            hook_stores,
            parent_node,
            last_node: node,
        }
    }
}

impl<RestStores, EntireStores, Func, Ret, Props>
    ComponentStoresWithNode<
        StoreCons<ComponentContainer<Func, Ret, Props>, RestStores>,
        EntireStores,
    >
where
    RestStores: StoresList,
    EntireStores: StoresList,
    Ret: ComponentReturn + 'static,
    Func: 'static + Fn(ComponentBuilder, Props) -> Ret,
    Props: PartialEq + Clone,
{
    pub fn sibling(
        self,
        component_func: Func,
        props: Props,
    ) -> ComponentStoresWithNode<RestStores, EntireStores> {
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

impl<ThisStore, RestStores, EntireStores>
    ComponentStoresWithNode<StoreCons<ThisStore, RestStores>, EntireStores>
where
    ThisStore: StoresList,
    RestStores: StoresList,
    EntireStores: StoresList,
{
    pub fn child<Builder>(
        self,
        builder: Builder,
    ) -> ComponentStoresWithNode<RestStores, EntireStores>
    where
        Builder: Fn(
            ComponentStores<ThisStore, ThisStore>,
        ) -> ComponentStoresWithNode<StoreConsEnd, ThisStore>,
    {
        let ComponentStoresWithNode {
            hook_stores,
            parent_node,
            last_node,
            ..
        } = self;
        let (rest_stores, store) = hook_stores.use_one_store();
        let comp_stores = ComponentStores {
            hook_stores: HookStores {
                current: store,
                entire: PhantomData,
                lock: rest_stores.lock.clone(),
                rerun: rest_stores.rerun.clone(),
            },
            parent_node: last_node.clone(),
        };
        builder(comp_stores);
        ComponentStoresWithNode {
            hook_stores: rest_stores,
            last_node,
            parent_node,
        }
    }
}
