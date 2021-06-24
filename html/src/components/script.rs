use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlScriptElement;
pub fn script(reia: ComponentBuilder, props: HtmlProps<HtmlScriptElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlScriptElement>,
        UseElementArgs {
            tag_name: "script",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
