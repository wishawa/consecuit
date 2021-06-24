use std::borrow::BorrowMut;

use crate::construction::{hook::HookBuilder, types::HookReturn};

use super::{use_ref, ReiaRef};

struct StoredEffect<Args, OnDrop>
where
    OnDrop: FnOnce() + 'static,
    Args: PartialEq + Clone + 'static,
{
    args: Args,
    on_drop: Option<OnDrop>,
}

impl<Args, OnDrop> Drop for StoredEffect<Args, OnDrop>
where
    OnDrop: FnOnce() + 'static,
    Args: PartialEq + Clone + 'static,
{
    fn drop(&mut self) {
        if let Some(od) = self.on_drop.take() {
            od();
        }
    }
}

/** Runs the function with the arg as argument when the arg changes.

Takes a single-argument function and the arg.

The arg must be [PartialEq] + [Clone] + `'static`, because we need to store and compare it.

The effect runs as the component renders. Updating states inside `use_effect` will queue another render.

If you want something to run after the component completed rendering, consider using [crate::executor::run_later].

This takes a function rather than a closure, so every dependency must be passed through `args`.
For React devs, this is equivalent to `react-hooks/exhaustive-deps` being enforced.

Example using this to change the page title when some data change:

```
let (reia, _) = reia.hook(use_effect, (
    |deps: (String, u32)| {
        let title = format!("Profile - {}, age {}", deps.0, deps.1);
        web_sys::window().unwrap().document().unwrap().set_title(&title);
    }, (name, number)
));

```

 */
pub fn use_effect<Args, OnDrop>(
    reia: HookBuilder,
    (func, args): (fn(Args) -> OnDrop, Args),
) -> impl HookReturn<()>
where
    OnDrop: FnOnce() + 'static,
    Args: PartialEq + Clone + 'static,
{
    use_effect_relaxed(reia, (func, args))
}

/** Like [use_effect], but takes a closure instead of a function.

This mean you don't have to pass every dependency through `args`.
For React devs, this is equivalent to `react-hooks/exhaustive-deps` not being enforced.
 */
pub fn use_effect_relaxed<Args, OnDrop, Eff>(
    reia: HookBuilder,
    (func, args): (Eff, Args),
) -> impl HookReturn<()>
where
    OnDrop: FnOnce() + 'static,
    Args: PartialEq + Clone + 'static,
    Eff: FnOnce(Args) -> OnDrop,
{
    let reia = reia.init();
    let (reia, store): (_, ReiaRef<Option<StoredEffect<Args, OnDrop>>>) = reia.hook(use_ref, ());
    let should_run = store
        .visit_mut_with(|opt| match opt {
            Some(stored) => {
                if stored.args != args {
                    stored.args = args;
                    true
                } else {
                    false
                }
            }
            opt_none => {
                *opt_none = Some(StoredEffect {
                    args,
                    on_drop: None,
                });
                true
            }
        })
        .unwrap();
    if should_run {
        store
            .visit_mut_with(|opt| {
                let stored = opt.as_mut().unwrap().borrow_mut();
                if let Some(od) = stored.on_drop.take() {
                    od();
                }
                stored.on_drop = Some(func(stored.args.clone()));
            })
            .unwrap();
    }
    (reia, ())
}
