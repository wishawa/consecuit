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
    fn hash(&self) -> (*const dyn ComponentStore, *const AtomicBool) {
        (self.comp, self.lock.as_ptr())
    }
    pub(crate) fn new(comp: &'static dyn ComponentStore, lock: UnmountedLock) -> Self {
        Self { comp, lock }
    }
    pub(crate) fn enqueue(self) {
        PENDING_RERENDERS.with(|p| {
            if p.borrow_mut().insert(self.hash()) {
                EXECUTOR.with(|l| l.enqueue(ExecutorTask::Rerender(self)))
            }
        });
    }
    fn execute(&self) {
        PENDING_RERENDERS.with(|p| {
            p.borrow_mut().remove(&self.hash());
        });
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
        (self.comp as *const _ == other.comp as *const _) && (self.lock == other.lock)
    }
}

thread_local! {
    static PENDING_RERENDERS: RefCell<HashSet<(*const dyn ComponentStore, *const AtomicBool)>> = RefCell::new(HashSet::new());
}

pub struct RunTask {
    run: Box<dyn FnOnce()>,
}

impl RunTask {
    pub fn new<F: FnOnce() + 'static>(func: F) -> Self {
        Self {
            run: Box::new(func),
        }
    }
    pub fn enqueue(self) {
        EXECUTOR.with(|l| l.enqueue(ExecutorTask::Run(self)))
    }
    fn execute(self) {
        (self.run)();
    }
}

enum ExecutorTask {
    Rerender(RerenderTask),
    Run(RunTask),
}

impl ExecutorTask {
    fn execute(self) {
        match self {
            ExecutorTask::Rerender(rerender) => rerender.execute(),
            ExecutorTask::Run(run) => run.execute(),
        }
    }
}

struct Executor {
    queue: RefCell<VecDeque<ExecutorTask>>,
    active: AtomicBool,
    timeout_closure: Closure<dyn Fn()>,
}

impl Executor {
    fn enqueue(&self, task: ExecutorTask) {
        self.queue.borrow_mut().push_back(task);
        self.maybe_batch_updates_with_timeout();
    }
    fn execute_loop(&self) {
        loop {
            let task_opt = { self.queue.borrow_mut().pop_front() };
            if let Some(task) = task_opt {
                task.execute();
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
    static EXECUTOR: Executor = Executor {
        active: AtomicBool::new(false),
        queue: RefCell::new(VecDeque::new()),
        timeout_closure: Closure::wrap(Box::new(|| {
            EXECUTOR.with(|l| {
                l.active.store(false, Ordering::SeqCst);
                l.execute();
            });
        }))
    };
}

pub fn batch_updates(f: impl FnOnce()) {
    EXECUTOR.with(|l| l.batch_updates(f))
}

pub fn run_later(f: impl FnOnce() + 'static) {
    RunTask::new(f).enqueue();
}
