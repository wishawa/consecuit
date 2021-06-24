use crate::{
    construction::{hook::HookBuilder, types::HookReturn},
    executor::RerenderTask,
};

use super::use_ref::{use_ref, ReiaRef};

/** A handle returned by [use_state] for updating the state.

Updating the state will schedule a rerender.

*/
#[derive(Clone, PartialEq)]
pub struct Updater<T: 'static> {
    state: ReiaRef<Option<T>>,
    rerender_task: RerenderTask,
}

impl<T> Updater<T> {
    /** Update the state value with the given closure.

    The closure takes the current value and returns the new value.

    This will schedule a rerender.

    */
    pub fn update_with<F: FnOnce(T) -> T>(&self, func: F) {
        self.state
            .visit_mut_with(|state| *state = Some(func(state.take().unwrap())))
            .unwrap();
        self.rerender_task.clone().enqueue();
    }

    /** Set the state value to the given value.

    This will schedule a rerender.

    */
    pub fn set_to(&self, value: T) {
        self.state
            .visit_mut_with(|state| *state = Some(value))
            .unwrap();
        self.rerender_task.clone().enqueue();
    }
}

/** Use a state. The given closure will be used to create the initial value.

Returns the value in the state, and an [Updater] that can be used to update it.

Example:

```
let (reia, (age, set_age)) = reia.hook(use_state_from, || {
    rand::random::<i32>()
});
```

Take a look at [the counters example](https://github.com/wishawa/reia/tree/main/examples/counters/src/lib.rs),
for example on using this to create a counter.
*/
pub fn use_state_from<T, F>(reia: HookBuilder, default_from: F) -> impl HookReturn<(T, Updater<T>)>
where
    T: Clone + 'static,
    F: FnOnce() -> T,
{
    let reia = reia.init();
    let (reia, state): (_, ReiaRef<Option<T>>) = reia.hook(use_ref::<Option<T>>, ());
    let value = state
        .visit_mut_with(|opt| opt.get_or_insert_with(default_from).clone())
        .unwrap();
    let setter = Updater {
        state,
        rerender_task: RerenderTask::new(reia.current_component, reia.lock.clone()),
    };
    (reia, (value, setter))
}

/** Use a state. The given value will be the default.

The value should be [Copy]. If it is not, consider using [use_state_from] instead.

Returns the value in the state, and an [Updater] that can be used to update it.

Example:

```
let (reia, (age, set_age)) = reia.hook(use_state, 3);
```
*/
pub fn use_state<T>(reia: HookBuilder, default_value: T) -> impl HookReturn<(T, Updater<T>)>
where
    T: Copy + Clone + 'static,
{
    use_state_from(reia, move || default_value)
}
