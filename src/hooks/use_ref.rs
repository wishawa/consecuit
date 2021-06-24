use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};

use crate::{
    construction::{hook::HookBuilder, types::HookReturn},
    locking::{SubtreeUnmountedError, UnmountedLock},
};

/** A reference with interior mutability. Somewhat like [RefCell]. Returned by [use_ref].

This can be read from and written to anytime. Writing to it does not trigger rerender.

This is often not neccessary. Use [super::use_state()] instead.
*/
#[derive(Clone)]
pub struct Reference<T: Default + 'static> {
    inside: &'static RefCell<T>,
    lock: UnmountedLock,
}

impl<T: Default + 'static> PartialEq for Reference<T> {
    fn eq(&self, other: &Reference<T>) -> bool {
        self.inside.as_ptr() == other.inside.as_ptr()
    }
}

impl<T: Default + 'static> Reference<T> {
    /** Run the given closure with a borrow of the data inside the Reference as argument.

    Returns a Result with the Ok variant being the return value of your closure.

    The error variant is [`SubtreeUnmountedError`][crate::locking::SubtreeUnmountedError].
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

    /** Run the given closure with a mutable borrow of the data inside the Reference as argument.

    Returns a Result with the Ok variant being the return value of your closure.

    The error variant is [`SubtreeUnmountedError`][crate::locking::SubtreeUnmountedError].
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

    The error variant is [`SubtreeUnmountedError`][crate::locking::SubtreeUnmountedError].
    This error is thrown if this function is called when the subtree where the [use_ref] this comes from had been unmounted.
    */
    pub fn set_in(&self, value: T) -> Result<(), SubtreeUnmountedError> {
        self.visit_mut_with(|v| {
            *v = value;
        })
    }
}

impl<T: Clone + Default + 'static> Reference<T> {
    /** Return a clone of the value inside the Ref.

    The Ok variant is said value.

    The error variant is [`SubtreeUnmountedError`].
    This error is thrown if this function is called when the subtree where the [use_ref] this comes from had been unmounted.
     */
    pub fn clone_out(&self) -> Result<T, SubtreeUnmountedError> {
        self.visit_with(|v| v.clone())
    }
}

/** Use a [Reference].

The components of the `consecuit_html` crate can take a reference as prop, giving you a handle to the underlying [web_sys] handle of the component.
See the `consecuit_html` crate documentations for more.

*/
pub fn use_ref<T>(cc: HookBuilder, _: ()) -> impl HookReturn<Reference<T>>
where
    T: Default + 'static,
{
    let cc = cc.init();
    let (cc, store): (_, &'static RefCell<T>) = cc.use_one_store();
    let reference = Reference {
        inside: store,
        lock: cc.lock.clone(),
    };
    (cc, reference)
}
