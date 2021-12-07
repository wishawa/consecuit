use std::{any::Any, marker::PhantomData};

use crate::{
    locking::SharedPart,
    stores::{StoreCons, StoresList},
};

use super::{component::ComponentStore, types::HookReturn};

/** The initial `consecuit` or `cc` object that every hook takes as first argument.

For more information on how to write hooks, see the docs at [crate].

*/
pub struct HookBuilder {
    pub(crate) untyped_stores: SharedPart<dyn Any>,
    pub(crate) current_component: SharedPart<dyn ComponentStore>,
}

impl HookBuilder {
    /// Make it ready to call `.hook(...)`.
    /// You shouldn't need this, as we have a shortbut that automatically call it when you call `.hook(...)`.
    pub fn init<T: StoresList>(self) -> HookConstruction<T, T> {
        /*
        Each Consecuit hook gets a type-erased `HookBuilder` as input.
        The actual input type is encoded in the called hook's `impl HookReturn<...>` return type.

        The caller (`cc.hook(...)`) and the called hook must agree on the return type of the called hook,
        so both know the called hook's true input type.

        The caller erases that type information from the input, but the called hook knows the type, so it unsafely cast back back correctly.

        Without this trick, each hook/component's signature would need to list out every state-slot it and its descendants use.
        */
        //let current: &T = unsafe { &*(self.untyped_stores as *const T) };
        let current: SharedPart<T> = self.untyped_stores.panicking_downcast();
        HookConstruction {
            current,
            entire: PhantomData,
            current_component: self.current_component,
        }
    }
}

/** This is the `consecuit` or `cc` object in your hook function.

You can use it to call other hooks.

You must return it at the end of your hook function. (See the doc at [`crate`] on how to write hooks).
 */
pub struct HookConstruction<CurrentStores: StoresList, EntireStores: StoresList> {
    pub(crate) current: SharedPart<CurrentStores>,
    pub(crate) entire: PhantomData<EntireStores>,
    pub(crate) current_component: SharedPart<dyn ComponentStore>,
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
        SharedPart<ThisStore>,
    ) {
        let Self { current, .. } = self;
        //let StoreCons(store, rest) = current;
        let store = current.clone().map(|s| &s.0);
        let rest = current.map(|s| &s.1);
        let new_rs = HookConstruction {
            current: rest,
            entire: PhantomData,
            current_component: self.current_component,
        };

        (new_rs, store)
    }
}

fn run_hook<Arg, Out, Ret>(
    store: SharedPart<Ret::StoresList>,
    current_component: SharedPart<dyn ComponentStore>,
    hook_func: fn(HookBuilder, Arg) -> Ret,
    hook_arg: Arg,
) -> Out
where
    Ret: HookReturn<Out>,
{
    //let untyped_stores = store as *const <Ret as HookReturn<Out>>::StoresList as *const ();
    let cc = HookBuilder {
        untyped_stores: store.upcast(),
        current_component,
    };
    let out: Out = hook_func(cc, hook_arg).get_val();
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

    Consumes `self`. Returns a tuple of `(cc, <return value of hook>)`.
    You can use the returned `cc` to call more hooks.

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
        let out = run_hook(store, rest_stores.current_component, hook_func, hook_arg);
        (rest_stores, out)
    }
}
