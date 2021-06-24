use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlLegendElement;
pub fn legend(cc: ComponentBuilder, props: HtmlProps<HtmlLegendElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlLegendElement>,
        UseElementArgs {
            tag_name: "legend",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
