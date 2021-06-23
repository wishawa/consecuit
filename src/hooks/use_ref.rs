use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};

use crate::{
    construction::{HookBuilder, HookReturn},
    unmounted_lock::UnmountedLock,
};

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
    pub fn visit_with<Ret: 'static, F: FnOnce(&T) -> Ret>(&self, func: F) -> Result<Ret, ()> {
        if self.lock.is_mounted() {
            Ok(func(self.inside.borrow().deref()))
        } else {
            Err(())
        }
    }
    pub fn visit_mut_with<Ret: 'static, F: FnOnce(&mut T) -> Ret>(
        &self,
        func: F,
    ) -> Result<Ret, ()> {
        if self.lock.is_mounted() {
            Ok(func(self.inside.borrow_mut().deref_mut()))
        } else {
            Err(())
        }
    }
    pub fn set_in(&self, value: T) -> Result<(), ()> {
        self.visit_mut_with(|v| {
            *v = value;
        })
    }
}

impl<T: Clone + Default + 'static> ReiaRef<T> {
    pub fn clone_out(&self) -> Result<T, ()> {
        self.visit_with(|v| v.clone())
    }
}

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
