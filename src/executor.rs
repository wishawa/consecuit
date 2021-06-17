use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    sync::atomic::{AtomicBool, Ordering},
};

use web_sys::console;

use crate::unmounted_lock::UnmountedLock;

#[derive(Clone)]
pub(crate) struct RerenderTask {
    pub(crate) obj: &'static dyn Renderable,
    pub(crate) lock: UnmountedLock,
}

impl RerenderTask {
    pub(crate) fn enqueue_self(&self) {
        EXECUTOR.enqueue(self.clone());
    }
    pub(crate) fn render(&self) {
        if self.lock.is_mounted() {
            self.obj.render();
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
        self.obj as *const _ == other.obj as *const _
    }
}

pub(crate) trait Renderable {
    fn render(&'static self);
}

struct Executor {
    queue: RefCell<VecDeque<RerenderTask>>,
    queue_set: RefCell<HashSet<*const dyn Renderable>>,
    lock: AtomicBool,
}

impl Executor {
    fn enqueue(&self, task: RerenderTask) {
        if self.queue_set.borrow_mut().insert(task.obj) {
            self.queue.borrow_mut().push_back(task);
        }
    }
    fn execute(&self) {
        if !self.lock.swap(true, Ordering::Acquire) {
            loop {
                let task_opt = { self.queue.borrow_mut().pop_front() };
                if let Some(task) = task_opt {
                    {
                        self.queue_set
                            .borrow_mut()
                            .remove(&(task.obj as *const dyn Renderable))
                    };
                    task.render();
                } else {
                    break;
                }
            }
            self.lock.store(false, Ordering::Release);
        }
    }
}

thread_local! {
    static LOCAL_EXECUTOR: Executor = Executor {
        queue: RefCell::new(VecDeque::new()),
        queue_set: RefCell::new(HashSet::new()),
        lock: AtomicBool::new(false)
    };
}

pub(crate) struct GlobalExecutor;

impl GlobalExecutor {
    pub(crate) fn execute(&self) {
        LOCAL_EXECUTOR.with(|l| l.execute());
    }
    pub(crate) fn enqueue(&self, task: RerenderTask) {
        LOCAL_EXECUTOR.with(|l| l.enqueue(task));
    }
}

pub(crate) static EXECUTOR: GlobalExecutor = GlobalExecutor;
