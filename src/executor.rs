use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    sync::atomic::{AtomicBool, Ordering},
};

use web_sys::console;

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
}

impl Executor {
    fn enqueue(&self, task: RerenderTask) {
        if self.queue_set.borrow_mut().insert(task.comp) {
            self.queue.borrow_mut().push_back(task);
            if !self.is_active() {
                self.execute();
            }
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
        if !self.active.swap(true, Ordering::Acquire) {
            self.execute_loop();
            self.active.store(false, Ordering::Release);
        }
    }
    fn is_active(&self) -> bool {
        self.active.load(Ordering::Acquire)
    }
    fn batched_updates(&self, f: impl FnOnce()) {
        let already_running = self.active.swap(true, Ordering::Acquire);
        f();
        if !already_running {
            self.execute_loop();
            self.active.store(false, Ordering::Release);
        }
    }
}

thread_local! {
    static LOCAL_EXECUTOR: Executor = Executor {
        active: AtomicBool::new(false),
        queue: RefCell::new(VecDeque::new()),
        queue_set: RefCell::new(HashSet::new())
    };
}

pub fn batched_updates(f: impl FnOnce()) {
    LOCAL_EXECUTOR.with(|l| l.batched_updates(f))
}
