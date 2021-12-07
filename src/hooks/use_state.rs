use crate::{
    construction::{hook::HookBuilder, types::HookReturn},
    executor::RerenderTask,
    locking::SubtreeUnmountedError,
};

use super::use_ref::{use_ref, Reference};

/** A handle returned by [use_state] for updating the state.

Updating the state will queue a rerender.

*/
#[derive(Clone, PartialEq)]
pub struct Updater<T: 'static> {
    state: Reference<Option<T>>,
    rerender_task: RerenderTask,
}

impl<T> Updater<T> {
    /** Update the state value with the given closure.

    The closure takes the current value and returns the new value.

    This will silently fail if the state no longer exist (i.e. the component had been unmounted).

    This will queue a rerender.
    */
    pub fn update_with<F: FnOnce(T) -> T>(&self, func: F) {
        self.update_with_nonsilent(func).ok();
    }

    /** Like the [`update_with`][Updater::update_with()] method, but returns result (no silent failure).

    */
    pub fn update_with_nonsilent<F: FnOnce(T) -> T>(
        &self,
        func: F,
    ) -> Result<(), SubtreeUnmountedError> {
        self.state
            .visit_mut_with(|state| *state = Some(func(state.take().unwrap())))?;
        self.rerender_task.clone().enqueue();
        Ok(())
    }

    /** Set the state value to the given value.

    This will silently fail if the state no longer exist (i.e. the component had been unmounted).

    This will queue a rerender.
    */
    pub fn set_to(&self, value: T) {
        self.set_to_nonsilent(value).ok();
    }

    /** Like the [`set_to`][Updater::set_to()] method, but returns result (no silent failure).

    */
    pub fn set_to_nonsilent(&self, value: T) -> Result<(), SubtreeUnmountedError> {
        self.state.visit_mut_with(|state| *state = Some(value))?;
        self.rerender_task.clone().enqueue();
        Ok(())
    }
}

/** Use a state. The given closure will be used to create the initial value.

Returns the value in the state, and an [Updater] that can be used to update it.

Example:

```
let (cc, (age, set_age)) = cc.hook(use_state_from, || {
    rand::random::<i32>()
});
```

Take a look at [the counters example](https://github.com/wishawa/consecuit/tree/main/examples/counters/src/lib.rs),
for example on using this to create a counter.
*/
pub fn use_state_from<T, F>(cc: HookBuilder, default_from: F) -> impl HookReturn<(T, Updater<T>)>
where
    T: Clone + 'static,
    F: FnOnce() -> T,
{
    let cc = cc.init();
    let (cc, state): (_, Reference<Option<T>>) = cc.hook(use_ref::<Option<T>>, ());
    let value = state
        .visit_mut_with(|opt| opt.get_or_insert_with(default_from).clone())
        .unwrap();
    let setter = Updater {
        state,
        rerender_task: RerenderTask::new(cc.current_component.clone()),
    };
    (cc, (value, setter))
}

/** Use a state. The given value will be the default.

The value should be [Copy]. If it is not, consider using [use_state_from] instead.

Returns the value in the state, and an [Updater] that can be used to update it.

Example:

```
let (cc, (age, set_age)) = cc.hook(use_state, 3);
```
*/
pub fn use_state<T>(cc: HookBuilder, default_value: T) -> impl HookReturn<(T, Updater<T>)>
where
    T: Copy + Clone + 'static,
{
    use_state_from(cc, move || default_value)
}
