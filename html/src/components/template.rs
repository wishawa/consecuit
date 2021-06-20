use crate::elem::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTemplateElement;
pub fn template(
    reia: ComponentBuilder,
    props: ElementProps<HtmlTemplateElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlTemplateElement>,
        UseElementArgs {
            tag_name: "template",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
