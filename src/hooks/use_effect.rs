use std::borrow::BorrowMut;

use crate::{HookBuilder, HookReturn};

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
