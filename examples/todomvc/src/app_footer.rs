use reia::*;
use reia_html::prelude::*;

fn footer_span(reia: ComponentBuilder, _: ()) -> impl ContainerReturn {
    reia.init()
        .comp(
            span,
            html_props().class_name("m-0.5 text-xs text-center text-gray-500"),
        )
        .hole_here()
}

fn hoverable_link(
    reia: ComponentBuilder,
    (text, href): (&'static str, &'static str),
) -> impl ComponentReturn {
    reia.init()
        .comp(
            a,
            HtmlProps::<web_sys::HtmlAnchorElement>::new()
                .class_name("hover:underline")
                .href(href),
        )
        .child(|r| r.comp(text_node, text))
}

pub fn app_footer(reia: ComponentBuilder, _: ()) -> impl ComponentReturn {
    reia.init()
        .comp(div, html_props().class_name("mt-16 p-4 flex flex-col"))
        .child(|r| {
            r.comp(footer_span, ())
                .child(|r| r.comp(text_node, "Double click to edit a todo"))
                .comp(footer_span, ())
                .child(|r| {
                    r.comp(text_node, "Written by ")
                        .comp(hoverable_link, ("Wisha Wa", "https://github.com/wishawa/"))
                })
                .comp(footer_span, ())
                .child(|r| {
                    r.comp(text_node, "on the ")
                        .comp(
                            hoverable_link,
                            ("Reia framework", "https://github.com/wishawa/reia"),
                        )
                        .comp(text_node, " with ")
                        .comp(hoverable_link, ("Tailwind CSS", "https://tailwindcss.com/"))
                })
                .comp(footer_span, ())
                .child(|r| {
                    r.comp(text_node, "to the ")
                        .comp(hoverable_link, ("TodoMVC", "https://todomvc.com/"))
                        .comp(text_node, " ")
                        .comp(
                            hoverable_link,
                            (
                                "specification",
                                "https://github.com/tastejs/todomvc/blob/master/app-spec.md",
                            ),
                        )
                })
        })
}
