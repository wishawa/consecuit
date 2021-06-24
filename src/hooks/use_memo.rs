use crate::construction::{hook::HookBuilder, types::HookReturn};

use super::{use_ref, Reference};

/** Memoize the computation.

Takes a single-argument function and the argument.

Return the return result of the function ran with the given argument.

The computation will be memoized; the function would only rerun when the arg chages.

The arg must be [PartialEq] + [Clone] + `'static`, because we need to store and compare it.

This takes a function rather than a closure, so every dependency must be passed through `args`.
For React devs, this is equivalent to `react-hooks/exhaustive-deps` being enforced.

Example using this to find the factors of a large number:

```
let (cc, factors) = cc.hook(use_memo, (
    |number: i32| {
        let factors = Vec::new();
        for i in 2..number {
            if number % i == 0 {
                factors.push(i);
            }
        }
        factors
    },
    number
))
```
 */
pub fn use_memo<Deps, Res>(
    cc: HookBuilder,
    (compute, deps): (fn(Deps) -> Res, Deps),
) -> impl HookReturn<Res>
where
    Deps: Clone + PartialEq + 'static,
    Res: Clone + 'static,
{
    use_memo_relaxed(cc, (compute, deps))
}

/** Like [use_memo], but takes a closure instead of a function.

This mean you don't have to pass every dependency through `args`.
For React devs, this is equivalent to `react-hooks/exhaustive-deps` not being enforced.

*/

pub fn use_memo_relaxed<Deps, Res, Compute: FnOnce(Deps) -> Res>(
    cc: HookBuilder,
    (compute, deps): (Compute, Deps),
) -> impl HookReturn<Res>
where
    Deps: Clone + PartialEq + 'static,
    Res: Clone + 'static,
{
    let cc = cc.init();
    let (cc, store): (_, Reference<Option<(Deps, Res)>>) = cc.hook(use_ref, ());
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
    (cc, res)
}
