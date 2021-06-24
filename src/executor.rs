use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    hash::Hash,
    sync::atomic::{AtomicBool, Ordering},
};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{console, window};

use crate::{construction::component::ComponentStore, unmounted_lock::UnmountedLock};

#[derive(Clone)]
pub(crate) struct RerenderTask {
    pub(crate) comp: &'static dyn ComponentStore,
    pub(crate) lock: UnmountedLock,
}

impl Hash for RerenderTask {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.comp as *const dyn ComponentStore, self.lock.as_ptr()).hash(state)
    }
}

impl PartialEq for RerenderTask {
    fn eq(&self, other: &RerenderTask) -> bool {
        (self.comp as *const _ as *const () == other.comp as *const _ as *const ())
            && (self.lock == other.lock)
    }
}

impl Eq for RerenderTask {}

impl RerenderTask {
    pub(crate) fn new(comp: &'static dyn ComponentStore, lock: UnmountedLock) -> Self {
        Self { comp, lock }
    }
    pub(crate) fn enqueue(self) {
        PENDING_RERENDERS.with(|p| {
            if p.borrow_mut().insert(self.clone()) {
                EXECUTOR.with(|l| l.enqueue(ExecutorTask::Rerender(self)))
            }
        });
    }
    fn execute(&self) {
        PENDING_RERENDERS.with(|p| {
            p.borrow_mut().remove(&self);
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

thread_local! {
    static PENDING_RERENDERS: RefCell<HashSet<RerenderTask>> = RefCell::new(HashSet::new());
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
        if !self.active.swap(true, Ordering::Acquire) {
            self.execute_loop();
            self.active.store(false, Ordering::Release);
        }
    }
    fn batch_updates(&self, f: impl FnOnce()) {
        let already_running = self.active.swap(true, Ordering::Acquire);
        f();
        if !already_running {
            self.active.store(false, Ordering::Release);
            self.execute();
        }
    }
    fn maybe_batch_updates_with_timeout(&self) {
        if !self.active.swap(true, Ordering::Acquire) {
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
                l.active.store(false, Ordering::Release);
                l.execute();
            });
        }))
    };
}

/** Batch the state updates started inside the given closure. **READ BEFORE USE**

Reia batch updates using setTimeout anyway, even if you are not using this.

Using this will avoid the setTimeout.

**Updates from inside [crate::use_effect], reia_html's Callback, and [run_later] are already batched.**

*/
pub fn batch_updates(f: impl FnOnce()) {
    EXECUTOR.with(|l| l.batch_updates(f))
}

/** Schedule the given closure to run after the current component finishes rendering.

Example use to focus a reia_html's input field:

```
use reia_html::prelude::*;
let (reia, input_ref) = reia.hook(use_ref, ());
let (reia, _) = reia.hook(use_effect, (|input_ref: ReiaRef<Option<web_sys::HtmlInputElement>>| {
    run_later(move || {
        input_ref.visit_with(|opt| opt.as_ref().unwrap().focus().unwrap()).unwrap();
    })
}, input_ref.clone()))
reia_tree!(
    <input html_props().reference(input_ref) />
)

```
*/
pub fn run_later(f: impl FnOnce() + 'static) {
    RunTask::new(f).enqueue();
}
