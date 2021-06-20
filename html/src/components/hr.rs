use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlHrElement;
pub fn hr(reia: ComponentBuilder, props: HtmlProps<HtmlHrElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlHrElement>,
        UseElementArgs {
            tag_name: "hr",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
