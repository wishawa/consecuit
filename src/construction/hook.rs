use std::{marker::PhantomData, mem::transmute};

use super::component::ComponentStore;
use super::types::HookReturn;
use crate::{
    stores::{StoreCons, StoresList},
    unmounted_lock::UnmountedLock,
};

pub struct HookBuilder {
    pub(crate) untyped_stores: &'static (),
    pub(crate) lock: UnmountedLock,
    pub(crate) current_component: &'static dyn ComponentStore,
}

impl HookBuilder {
    pub fn init<T: StoresList>(self) -> HookConstruction<T, T> {
        let current: &T = unsafe { transmute::<&'static (), &'static T>(self.untyped_stores) };
        HookConstruction {
            current,
            entire: PhantomData,
            lock: self.lock,
            current_component: self.current_component,
        }
    }
}

pub struct HookConstruction<CurrentStores: StoresList, EntireStores: StoresList> {
    pub(crate) current: &'static CurrentStores,
    pub(crate) entire: PhantomData<EntireStores>,
    pub(crate) lock: UnmountedLock,
    pub(crate) current_component: &'static dyn ComponentStore,
}

impl<ThisStore, RestStores, EntireStores>
    HookConstruction<StoreCons<ThisStore, RestStores>, EntireStores>
where
    ThisStore: Default + 'static,
    RestStores: StoresList,
    EntireStores: StoresList,
{
    pub(crate) fn use_one_store(
        self,
    ) -> (
        HookConstruction<RestStores, EntireStores>,
        &'static ThisStore,
    ) {
        let Self { current, .. } = self;
        let StoreCons(store, rest) = current;
        let new_rs = HookConstruction {
            current: rest,
            entire: PhantomData,
            lock: self.lock,
            current_component: self.current_component,
        };

        (new_rs, store)
    }
}

fn run_hook<Arg, Out, Ret>(
    store: &'static Ret::StoresList,
    lock: UnmountedLock,
    current_component: &'static dyn ComponentStore,
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
        current_component,
    };
    let out: Out = hook_func(reia, hook_arg).get_val();
    out
}

impl<ThisStore, RestStores, EntireStores>
    HookConstruction<StoreCons<ThisStore, RestStores>, EntireStores>
where
    ThisStore: StoresList,
    RestStores: StoresList,
    EntireStores: StoresList,
{
    pub fn hook<Arg, Out, Ret>(
        self,
        hook_func: fn(HookBuilder, Arg) -> Ret,
        hook_arg: Arg,
    ) -> (HookConstruction<RestStores, EntireStores>, Out)
    where
        Ret: HookReturn<Out, StoresList = ThisStore>,
    {
        let (rest_stores, store) = self.use_one_store();
        let out = run_hook(
            store,
            rest_stores.lock.clone(),
            rest_stores.current_component,
            hook_func,
            hook_arg,
        );
        (rest_stores, out)
    }
}
