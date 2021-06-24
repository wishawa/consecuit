use web_sys::{window, Node};

use super::{
    component::ComponentBuilder,
    subtree::{mount_subtree, SubtreeInstance},
    types::ComponentReturn,
};

/// Mount the given component on the <body> element. This passes <body> to [`mount_app_at`].
///
/// The component must be a Consecuit component that takes `()` (empty tuple) as props.
///
/// See [`crate`] for what a Consecuit component looks like.
pub fn mount_app<Ret>(function: fn(ComponentBuilder, ()) -> Ret)
where
    Ret: ComponentReturn,
{
    let body = window().unwrap().document().unwrap().body().unwrap().into();
    mount_app_at(function, body)
}

/// Mount the given component on the given [`Node`]. This uses [`mount_app_without_leaking_at`] and leaks the result.
///
/// The component must be a Consecuit component that takes `()` (empty tuple) as props.
///
/// See [`crate`] for what a Consecuit component looks like.
pub fn mount_app_at<Ret>(function: fn(ComponentBuilder, ()) -> Ret, node: Node)
where
    Ret: ComponentReturn,
{
    Box::leak(Box::new(unsafe {
        mount_app_without_leaking_at(function, node)
    }));
}

/// Mount the given component on the given [`Node`], returning a [`SubtreeInstance`][crate::construction::subtree::SubtreeInstance] which will unmount the app when dropped.
///
/// Normally you should use [`mount_app`] or [`mount_app_at`]. Only use this if you have a way to store the returned value.
///
/// The component must be a Consecuit component that takes `()` (empty tuple) as props.
///
/// See [`crate`] for what a Consecuit component looks like.
///
/// This is unsafe because dropping the returned [`SubtreeInstance`][crate::construction::subtree::SubtreeInstance] while Consecuit is rendering could cause unsafe behavior.
///
/// If you're going to use it, make sure you keep the [`SubtreeInstance`][crate::construction::subtree::SubtreeInstance] forever.
///
/// If you're really going to drop it, make sure to NOT do so from inside Consecuit hooks/components/use_effect/....
/// Dropping from inside an event callback or [`run_later`][crate::executor::run_later] is probably fine.
pub unsafe fn mount_app_without_leaking_at<Ret>(
    function: fn(ComponentBuilder, ()) -> Ret,
    node: Node,
) -> SubtreeInstance<Ret, ()>
where
    Ret: ComponentReturn,
{
    let root_tree = mount_subtree(function, (), node);

    root_tree
}
