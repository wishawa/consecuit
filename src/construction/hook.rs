use std::marker::PhantomData;

use crate::{
    locking::UnmountedLock,
    stores::{StoreCons, StoresList},
};

use super::{component::ComponentStore, types::HookReturn};

/** The initial `reia` object that every hook takes as first argument.

For more information on how to write hooks, see the docs at [crate].

*/
pub struct HookBuilder {
    pub(crate) untyped_stores: *const (),
    pub(crate) lock: UnmountedLock,
    pub(crate) current_component: &'static dyn ComponentStore,
}

impl HookBuilder {
    /// Make it ready to call `.hook(...)`.
    /// You shouldn't need this, as we have a shortbut that automatically call it when you call `.hook(...)`.
    pub fn init<T: StoresList>(self) -> HookConstruction<T, T> {
        let current: &T = unsafe { &*(self.untyped_stores as *const T) };
        HookConstruction {
            current,
            entire: PhantomData,
            lock: self.lock,
            current_component: self.current_component,
        }
    }
}

/** This is the `reia` object in your hook function.

You can use it to call other hooks.

You must return it at the end of your hook function. (See the doc for [`crate`] on how to write hooks).
 */
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
    let untyped_stores = store as *const <Ret as HookReturn<Out>>::StoresList as *const ();
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
    /** Use the given hook, with the given arg.

    Consumes `self`. Returns a tuple of `(reia, <return value of hook>)`.
    You can use the returned `reia` to call more hooks.

    See the docs at [crate] for more info on how to write and use hooks.
     */
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
