use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    sync::atomic::{AtomicBool, Ordering},
};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{console, window};

use crate::{component::ComponentStore, unmounted_lock::UnmountedLock};

#[derive(Clone)]
pub(crate) struct RerenderTask {
    pub(crate) comp: &'static dyn ComponentStore,
    pub(crate) lock: UnmountedLock,
}

impl RerenderTask {
    pub(crate) fn new(comp: &'static dyn ComponentStore, lock: UnmountedLock) -> Self {
        Self { comp, lock }
    }
    pub(crate) fn enqueue_self(&self) {
        LOCAL_EXECUTOR.with(|l| l.enqueue(self.clone()))
    }
    pub(crate) fn render(&self) {
        if self.lock.is_mounted() {
            self.comp.render();
        } else {
            console::warn_1(
                &"Trying to render a component whose tree had been unmounted. This is a no-op."
                    .into(),
            );
        }
    }
}

impl PartialEq for RerenderTask {
    fn eq(&self, other: &RerenderTask) -> bool {
        self.comp as *const _ == other.comp as *const _
    }
}

struct Executor {
    queue: RefCell<VecDeque<RerenderTask>>,
    queue_set: RefCell<HashSet<*const dyn ComponentStore>>,
    active: AtomicBool,
    timeout_closure: Closure<dyn Fn()>,
}

impl Executor {
    fn enqueue(&self, task: RerenderTask) {
        if self.queue_set.borrow_mut().insert(task.comp) {
            self.queue.borrow_mut().push_back(task);
            self.maybe_batch_updates_with_timeout();
        }
    }
    fn execute_loop(&self) {
        loop {
            let task_opt = { self.queue.borrow_mut().pop_front() };
            if let Some(task) = task_opt {
                {
                    self.queue_set
                        .borrow_mut()
                        .remove(&(task.comp as *const dyn ComponentStore))
                };
                task.render();
            } else {
                break;
            }
        }
    }
    fn execute(&self) {
        if !self.active.swap(true, Ordering::SeqCst) {
            self.execute_loop();
            self.active.store(false, Ordering::SeqCst);
        }
    }
    fn batch_updates(&self, f: impl FnOnce()) {
        let already_running = self.active.swap(true, Ordering::SeqCst);
        f();
        if !already_running {
            self.active.store(false, Ordering::SeqCst);
            self.execute();
        }
    }
    fn maybe_batch_updates_with_timeout(&self) {
        if !self.active.swap(true, Ordering::SeqCst) {
            window()
                .unwrap()
                .set_timeout_with_callback(self.timeout_closure.as_ref().unchecked_ref())
                .unwrap();
        }
    }
}

thread_local! {
    static LOCAL_EXECUTOR: Executor = Executor {
        active: AtomicBool::new(false),
        queue: RefCell::new(VecDeque::new()),
        queue_set: RefCell::new(HashSet::new()),
        timeout_closure: Closure::wrap(Box::new(|| {
            LOCAL_EXECUTOR.with(|l| {
                l.active.store(false, Ordering::SeqCst);
                l.execute();
            });
        }))
    };
}

pub fn batch_updates(f: impl FnOnce()) {
    LOCAL_EXECUTOR.with(|l| l.batch_updates(f))
}
