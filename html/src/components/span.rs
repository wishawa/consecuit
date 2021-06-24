use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlSpanElement;
pub fn span(reia: ComponentBuilder, props: HtmlProps<HtmlSpanElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlSpanElement>,
        UseElementArgs {
            tag_name: "span",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
