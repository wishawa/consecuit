use crate::{
    construction::{HookBuilder, HookReturn},
    executor::RerenderTask,
};

use super::use_ref::{use_ref, ReiaRef};

#[derive(Clone, PartialEq)]
pub struct StateSetter<T: 'static> {
    state: ReiaRef<Option<T>>,
    rerender_task: RerenderTask,
}

impl<T> StateSetter<T> {
    pub fn set(&self, value: T) {
        self.state
            .visit_mut_with(|state| *state = Some(value))
            .unwrap();
        self.rerender_task.clone().enqueue();
    }
}
impl<T: Clone> StateSetter<T> {
    pub fn update_with<F: FnOnce(T) -> T>(&self, func: F) {
        self.state
            .visit_mut_with(|state| *state = Some(func(state.clone().unwrap())))
            .unwrap();
        self.rerender_task.clone().enqueue();
    }
}

pub fn use_state_from<T, F>(
    reia: HookBuilder,
    default_from: F,
) -> impl HookReturn<(T, StateSetter<T>)>
where
    T: Clone + 'static,
    F: FnOnce() -> T,
{
    let reia = reia.init();
    let (reia, state): (_, ReiaRef<Option<T>>) = reia.hook(use_ref::<Option<T>>, ());
    let value = state
        .visit_mut_with(|opt| opt.get_or_insert_with(default_from).clone())
        .unwrap();
    let setter = StateSetter {
        state,
        rerender_task: RerenderTask::new(reia.current_component, reia.lock.clone()),
    };
    (reia, (value, setter))
}

pub fn use_state<T>(reia: HookBuilder, default_value: T) -> impl HookReturn<(T, StateSetter<T>)>
where
    T: Clone + 'static,
{
    use_state_from(reia, || default_value)
}
