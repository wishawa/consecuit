use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};

use crate::{
    construction::{hook::HookBuilder, types::HookReturn},
    unmounted_lock::{SubtreeUnmountedError, UnmountedLock},
};

/** A reference with interior mutability. Somewhat like [RefCell].

This can be read from and written to anytime. Writing to it does not trigger rerender.

This is often not neccessary. Use [super::use_state] instead.
*/
#[derive(Clone)]
pub struct ReiaRef<T: Default + 'static> {
    inside: &'static RefCell<T>,
    lock: UnmountedLock,
}

impl<T: Default + 'static> PartialEq for ReiaRef<T> {
    fn eq(&self, other: &ReiaRef<T>) -> bool {
        self.inside.as_ptr() == other.inside.as_ptr()
    }
}

impl<T: Default + 'static> ReiaRef<T> {
    /** Run the given closure with a borrow of the data inside the ReiaRef as argument.

    Returns a Result with the Ok variant being the return value of your closure.

    The error variant is [`SubtreeUnmountedError`].
    This error is thrown if this function is called when the subtree where the [use_ref] this comes from had been unmounted.
    */
    pub fn visit_with<Ret: 'static, F: FnOnce(&T) -> Ret>(
        &self,
        func: F,
    ) -> Result<Ret, SubtreeUnmountedError> {
        if self.lock.is_mounted() {
            Ok(func(self.inside.borrow().deref()))
        } else {
            Err(SubtreeUnmountedError)
        }
    }

    /** Run the given closure with a mutable borrow of the data inside the ReiaRef as argument.

    Returns a Result with the Ok variant being the return value of your closure.

    The error variant is [`SubtreeUnmountedError`].
    This error is thrown if this function is called when the subtree where the [use_ref] this comes from had been unmounted.
    */
    pub fn visit_mut_with<Ret: 'static, F: FnOnce(&mut T) -> Ret>(
        &self,
        func: F,
    ) -> Result<Ret, SubtreeUnmountedError> {
        if self.lock.is_mounted() {
            Ok(func(self.inside.borrow_mut().deref_mut()))
        } else {
            Err(SubtreeUnmountedError)
        }
    }

    /** Set the value in the Ref to the given value.

    This does not trigger a rerender.

    The error variant is [`SubtreeUnmountedError`].
    This error is thrown if this function is called when the subtree where the [use_ref] this comes from had been unmounted.
    */
    pub fn set_in(&self, value: T) -> Result<(), SubtreeUnmountedError> {
        self.visit_mut_with(|v| {
            *v = value;
        })
    }
}

impl<T: Clone + Default + 'static> ReiaRef<T> {
    /** Return a clone of the value inside the Ref.

    The Ok variant is said value.

    The error variant is [`SubtreeUnmountedError`].
    This error is thrown if this function is called when the subtree where the [use_ref] this comes from had been unmounted.
     */
    pub fn clone_out(&self) -> Result<T, SubtreeUnmountedError> {
        self.visit_with(|v| v.clone())
    }
}

/** Use a [ReiaRef].

The components of the `reia_html` crate can take a reference as prop, giving you a handle to the underlying [web_sys] handle of the component.
See the `reia_html` crate documentations for more.

*/
pub fn use_ref<T>(reia: HookBuilder, _: ()) -> impl HookReturn<ReiaRef<T>>
where
    T: Default + 'static,
{
    let reia = reia.init();
    let (reia, store): (_, &'static RefCell<T>) = reia.use_one_store();
    let reia_ref = ReiaRef {
        inside: store,
        lock: reia.lock.clone(),
    };
    (reia, reia_ref)
}
