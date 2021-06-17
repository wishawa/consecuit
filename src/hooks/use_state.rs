use crate::hook::{HookBuilder, HookReturn, Rerunner};

use super::use_ref::{use_ref, ReiaRef};

#[derive(Clone, PartialEq)]
pub struct StateSetter<T: 'static> {
    state: ReiaRef<Option<T>>,
    rerun: Rerunner,
}

impl<T> StateSetter<T> {
    pub fn set(&self, value: T) {
        self.state
            .visit_mut_with(|state| *state = Some(value))
            .unwrap();
        self.rerun.rerun();
    }
}
impl<T: Clone> StateSetter<T> {
    pub fn update_with<F: FnOnce(T) -> T>(&self, func: F) {
        self.state
            .visit_mut_with(|state| *state = Some(func(state.clone().unwrap())))
            .unwrap();
        self.rerun.rerun();
    }
}

pub fn use_state<T>(reia: HookBuilder, default_value: T) -> impl HookReturn<(T, StateSetter<T>)>
where
    T: Copy + 'static,
{
    let reia = reia.init();
    let (reia, state): (_, ReiaRef<Option<T>>) = reia.hook(use_ref::<Option<T>>, ());
    let value = state
        .visit_mut_with(|opt| opt.get_or_insert(default_value).clone())
        .unwrap();
    let setter = StateSetter {
        state,
        rerun: reia.rerun.clone(),
    };
    (reia, (value, setter))
}
