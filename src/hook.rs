use std::{marker::PhantomData, mem::transmute};

use crate::{
    stores::{StoreCons, StoreConsEnd, StoresList},
    unmounted_lock::UnmountedLock,
};

#[derive(Clone)]
pub(crate) struct Rerunner(pub(crate) &'static dyn Rerun);

impl Rerunner {
    pub fn rerun(&self) {
        self.0.rerun();
    }
}

pub(crate) trait Rerun {
    fn rerun(&'static self);
}

pub struct HookBuilder {
    pub(crate) untyped_stores: &'static (),
    pub(crate) lock: UnmountedLock,
    pub(crate) rerun: Rerunner,
}

impl HookBuilder {
    pub fn init<T: StoresList>(self) -> HookStores<T, T> {
        let current: &T = unsafe { transmute::<&'static (), &'static T>(self.untyped_stores) };
        HookStores {
            current,
            entire: PhantomData,
            lock: self.lock,
            rerun: self.rerun,
        }
    }
}

pub struct HookStores<CurrentStores: StoresList, EntireStores: StoresList> {
    pub(crate) current: &'static CurrentStores,
    pub(crate) entire: PhantomData<EntireStores>,
    pub(crate) lock: UnmountedLock,
    pub(crate) rerun: Rerunner,
}

type EmptyHookStores<Entire> = HookStores<StoreConsEnd, Entire>;

pub trait HookValue<Value> {
    type StoresList: StoresList;
    fn get_val(self) -> Value;
}

impl<UsedStores, Value> HookValue<Value> for (EmptyHookStores<UsedStores>, Value)
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
            rerun: self.rerun,
        };

        (new_rs, store)
    }
    pub(crate) fn skip_one_store(self) -> HookStores<RestStores, EntireStores> {
        let Self { current, .. } = self;
        let StoreCons(_, r) = current;
        HookStores {
            current: r,
            entire: PhantomData,
            lock: self.lock,
            rerun: self.rerun,
        }
    }
}

fn run_hook<Func, Arg, Out, Ret>(
    store: &'static Ret::StoresList,
    lock: UnmountedLock,
    rerun: Rerunner,
    hook_func: Func,
    hook_arg: Arg,
) -> Out
where
    Ret: HookValue<Out>,
    Func: 'static + Fn(HookBuilder, Arg) -> Ret,
{
    let untyped_stores = unsafe { transmute::<&'static Ret::StoresList, &'static ()>(store) };
    let reia = HookBuilder {
        untyped_stores,
        lock,
        rerun,
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
    pub fn hook<Func, Arg, Out, Ret>(
        self,
        hook_func: Func,
        hook_arg: Arg,
    ) -> (HookStores<RestStores, EntireStores>, Out)
    where
        Ret: HookValue<Out, StoresList = NextStores>,
        Func: 'static + Fn(HookBuilder, Arg) -> Ret,
    {
        let (rest_stores, store) = self.use_one_store();
        let out = run_hook(
            store,
            rest_stores.lock.clone(),
            rest_stores.rerun.clone(),
            hook_func,
            hook_arg,
        );
        (rest_stores, out)
    }
}
