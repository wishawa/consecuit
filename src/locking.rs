/*! Locking mechanism. Don't mind this.

*/

use std::{
    any::Any,
    error::Error,
    fmt::Display,
    ops::Deref,
    rc::Rc,
    sync::atomic::{AtomicBool, Ordering},
};

use owning_ref::RcRef;

// #[derive(Clone)]
// pub(crate) struct UnmountedLock(Rc<AtomicBool>);

pub(crate) struct WithLock<T: ?Sized> {
    mounted: AtomicBool,
    data: T,
}

pub(crate) struct SharedPart<T: ?Sized + 'static>(pub(crate) RcRef<WithLock<dyn Any>, T>);

impl<T: ?Sized + 'static> Clone for SharedPart<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ?Sized + 'static> Deref for SharedPart<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl<T: ?Sized + 'static> SharedPart<T> {
    pub(crate) fn is_mounted(&self) -> bool {
        self.0.as_owner().mounted.load(Ordering::Acquire)
    }
    pub(crate) fn unmount_tree(&self) {
        self.0.as_owner().mounted.store(false, Ordering::Release)
    }
    pub(crate) fn map<U: ?Sized, F: FnOnce(&T) -> &U>(self, op: F) -> SharedPart<U> {
        SharedPart::<U>(self.0.map(op))
    }
}

impl SharedPart<dyn Any> {
    pub(crate) fn panicking_downcast<T>(self) -> SharedPart<T> {
        self.map(|a| a.downcast_ref().unwrap())
    }
}

impl<T: ?Sized + 'static> PartialEq for SharedPart<T> {
    fn eq(&self, other: &Self) -> bool {
        let s = &(**self);
        let o = &(**other);
        std::ptr::eq(s as *const T, o as *const T)
    }
}

impl<T: 'static> SharedPart<T> {
    pub(crate) fn new(data: T) -> Self {
        let rc = Rc::new(WithLock {
            mounted: AtomicBool::new(true),
            data,
        });
        Self(RcRef::new(rc as Rc<WithLock<dyn Any>>).map(|p| p.data.downcast_ref().unwrap()))
    }
    pub(crate) fn upcast(self) -> SharedPart<dyn Any> {
        self.map(|f| (f as &dyn Any))
    }
}

// impl UnmountedLock {
//     pub(crate) fn new_mounted() -> Self {
//         Self(Rc::new(AtomicBool::new(true)))
//     }
//     pub(crate) fn is_mounted(&self) -> bool {
//         self.0.load(Ordering::Acquire)
//     }
//     pub(crate) fn unmount(&self) {
//         self.0.store(false, Ordering::Release)
//     }
//     pub(crate) fn as_ptr(&self) -> *const AtomicBool {
//         Rc::as_ptr(&self.0)
//     }
// }

/** A failure where the subtree you are trying to do something with had been unmounted.

For example, [crate::hooks::Reference] have many methods that may error out with this.

*/
#[derive(Debug)]
pub struct SubtreeUnmountedError;

impl Display for SubtreeUnmountedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Attempted to use an unmounted subtree.")
    }
}

impl Error for SubtreeUnmountedError {}
