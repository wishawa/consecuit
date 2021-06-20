use crate::{HookBuilder, HookReturn};

use super::{use_ref, ReiaRef};

pub fn use_memo_relaxed<Deps, Res, Compute: FnOnce(Deps) -> Res>(
    reia: HookBuilder,
    (compute, deps): (Compute, Deps),
) -> impl HookReturn<Res>
where
    Deps: Clone + PartialEq + 'static,
    Res: Clone + 'static,
{
    let reia = reia.init();
    let (reia, store): (_, ReiaRef<Option<(Deps, Res)>>) = reia.hook(use_ref, ());
    let res = store
        .visit_mut_with(|opt| match opt {
            Some(stored) => {
                if stored.0 != deps {
                    stored.1 = compute(deps);
                }
                stored.1.clone()
            }
            opt_none => {
                let res = compute(deps.clone());
                *opt_none = Some((deps, res.clone()));
                res
            }
        })
        .unwrap();
    (reia, res)
}

pub fn use_memo<Deps, Res>(
    reia: HookBuilder,
    (compute, deps): (fn(Deps) -> Res, Deps),
) -> impl HookReturn<Res>
where
    Deps: Clone + PartialEq + 'static,
    Res: Clone + 'static,
{
    use_memo_relaxed(reia, (compute, deps))
}
