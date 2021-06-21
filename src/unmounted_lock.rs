use std::{
    rc::Rc,
    sync::atomic::{AtomicBool, Ordering},
};

#[derive(Clone)]
pub(crate) struct UnmountedLock(Rc<AtomicBool>);

impl PartialEq for UnmountedLock {
    fn eq(&self, other: &UnmountedLock) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl UnmountedLock {
    pub(crate) fn new_mounted() -> Self {
        Self(Rc::new(AtomicBool::new(true)))
    }
    pub(crate) fn is_mounted(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }
    pub(crate) fn unmount(&self) {
        self.0.store(false, Ordering::Release)
    }
    pub(crate) fn as_ptr(&self) -> *const AtomicBool {
        Rc::as_ptr(&self.0)
    }
}
