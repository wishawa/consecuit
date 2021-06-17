use std::borrow::BorrowMut;

use crate::{HookBuilder, HookReturn};

use super::{use_ref, ReiaRef};

pub trait RunOnDrop: 'static {
    fn run(self);
}

impl<F: FnOnce() + 'static> RunOnDrop for F {
    fn run(self) {
        self();
    }
}

impl RunOnDrop for () {
    fn run(self) {}
}

struct StoredEffect<Args, OnDrop>
where
    OnDrop: RunOnDrop,
    Args: PartialEq + Clone + 'static,
{
    args: Args,
    on_drop: Option<OnDrop>,
}

pub fn use_effect<Effect, Args, OnDrop>(
    reia: HookBuilder,
    (func, args): (&Effect, Args),
) -> impl HookReturn<()>
where
    OnDrop: RunOnDrop,
    Effect: Fn(Args) -> OnDrop + 'static,
    Args: PartialEq + Clone + 'static,
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
                    od.run();
                }
                stored.on_drop = Some(func(stored.args.clone()));
            })
            .unwrap();
    }
    (reia, ())
}
