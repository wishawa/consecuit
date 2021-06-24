use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTableElement;
pub fn table(reia: ComponentBuilder, props: HtmlProps<HtmlTableElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlTableElement>,
        UseElementArgs {
            tag_name: "table",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
