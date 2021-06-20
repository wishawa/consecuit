use crate::elem::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlFieldSetElement;
pub fn fieldset(
    reia: ComponentBuilder,
    props: ElementProps<HtmlFieldSetElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlFieldSetElement>,
        UseElementArgs {
            tag_name: "fieldset",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
