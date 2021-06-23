use web_sys::{window, Element};

use crate::{ComponentBuilder, ComponentReturn};

use super::subtree::{mount_subtree, Subtree, SubtreeInstance};

pub fn mount_app<Ret>(function: fn(ComponentBuilder, ()) -> Ret)
where
    Ret: ComponentReturn,
{
    let body = window().unwrap().document().unwrap().body().unwrap().into();
    mount_app_at(function, body)
}

pub fn mount_app_at<Ret>(function: fn(ComponentBuilder, ()) -> Ret, element: Element)
where
    Ret: ComponentReturn,
{
    Box::leak(Box::new(mount_app_without_leaking_at(function, element)));
}

pub fn mount_app_without_leaking_at<Ret>(
    function: fn(ComponentBuilder, ()) -> Ret,
    element: Element,
) -> SubtreeInstance<Ret, ()>
where
    Ret: ComponentReturn,
{
    let root_tree = mount_subtree(function, (), element);
    root_tree.re_render(());

    root_tree
}
