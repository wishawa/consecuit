use std::{marker::PhantomData, mem::transmute};

use crate::{
    executor::RerenderTask,
    stores::{StoreCons, StoreConsEnd, StoresList},
    unmounted_lock::UnmountedLock,
};

pub struct HookBuilder {
    pub(crate) untyped_stores: &'static (),
    pub(crate) lock: UnmountedLock,
    pub(crate) rerender_parent: RerenderTask,
}

impl HookBuilder {
    pub fn init<T: StoresList>(self) -> HookStores<T, T> {
        let current: &T = unsafe { transmute::<&'static (), &'static T>(self.untyped_stores) };
        HookStores {
            current,
            entire: PhantomData,
            lock: self.lock,
            rerender_parent: self.rerender_parent,
        }
    }
}

pub struct HookStores<CurrentStores: StoresList, EntireStores: StoresList> {
    pub(crate) current: &'static CurrentStores,
    pub(crate) entire: PhantomData<EntireStores>,
    pub(crate) lock: UnmountedLock,
    pub(crate) rerender_parent: RerenderTask,
}

type EmptyHookStores<Entire> = HookStores<StoreConsEnd, Entire>;

pub trait HookReturn<Value> {
    type StoresList: StoresList;
    fn get_val(self) -> Value;
}

impl<UsedStores, Value> HookReturn<Value> for (EmptyHookStores<UsedStores>, Value)
where
    UsedStores: StoresList,
{
    type StoresList = UsedStores;
    fn get_val(self) -> Value {
        self.1
    }
}

impl<ThisStore, RestStores, EntireStores> HookStores<StoreCons<ThisStore, RestStores>, EntireStores>
where
    ThisStore: Default + 'static,
    RestStores: StoresList,
    EntireStores: StoresList,
{
    pub(crate) fn use_one_store(
        self,
    ) -> (HookStores<RestStores, EntireStores>, &'static ThisStore) {
        let Self { current, .. } = self;
        let StoreCons(store, rest) = current;
        let new_rs = HookStores {
            current: rest,
            entire: PhantomData,
            lock: self.lock,
            rerender_parent: self.rerender_parent,
        };

        (new_rs, store)
    }
}

fn run_hook<Arg, Out, Ret>(
    store: &'static Ret::StoresList,
    lock: UnmountedLock,
    rerender_parent: RerenderTask,
    hook_func: fn(HookBuilder, Arg) -> Ret,
    hook_arg: Arg,
) -> Out
where
    Ret: HookReturn<Out>,
{
    let untyped_stores = unsafe { transmute::<&'static Ret::StoresList, &'static ()>(store) };
    let reia = HookBuilder {
        untyped_stores,
        lock,
        rerender_parent,
    };
    let out: Out = hook_func(reia, hook_arg).get_val();
    out
}

impl<NextStores, RestStores, EntireStores>
    HookStores<StoreCons<NextStores, RestStores>, EntireStores>
where
    NextStores: StoresList,
    RestStores: StoresList,
    EntireStores: StoresList,
{
    pub fn hook<Arg, Out, Ret>(
        self,
        hook_func: fn(HookBuilder, Arg) -> Ret,
        hook_arg: Arg,
    ) -> (HookStores<RestStores, EntireStores>, Out)
    where
        Ret: HookReturn<Out, StoresList = NextStores>,
    {
        let (rest_stores, store) = self.use_one_store();
        let out = run_hook(
            store,
            rest_stores.lock.clone(),
            rest_stores.rerender_parent.clone(),
            hook_func,
            hook_arg,
        );
        (rest_stores, out)
    }
}
