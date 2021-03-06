/*! Macros to make Consecuit easier to use.

There is currently only one macro: [cc_tree()]. It let you write components in JSX style.
*/

use std::convert::TryFrom;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote, ExprBlock, ExprPath, Token,
};
use syn_rsx::Node;

/** Macro to let you write UI in JSX style.

This:

```
cc_tree!(
    <div>
        "This is a text node"
        {format!("hi: {}", name)}
        <span {html_props().class_name("text-span")}>"This is a text node in a span"</span>
        <my_component { MyComponentProps { value: 5 } } />
    </div>
)
```

expands to:

```
cc.comp(div, Default::default())
.child(|cc| {
    cc.comp(text_node, "This is a text node")
    .comp(text_node, { format!("hi: {}", name) })
    .comp(span, html_props().class_name("text-span"))
    .child(|cc| {
        cc.comp(text_node, "This is a text node in a span")
    })
    .comp(my_component, MyComponentProps { value: 5 })
})
```

As you might have figured out from above already:

* Use the name of the component function is the name of the tag.
* Put the props in curly braces beside the tag name.
If there are none, the macro will attempt to use the [`Default::default`] value (and error if the props doesn't implement Default).
* Use string literal or any [`AsRef<str>`] type in braces to create text node.


If you use names other than `cc` for your Consecuit object, you can specify it as first arg to the macro:

```
fn my_comp(consecuit_object: ComponentBuilder, _props: ()) -> impl ComponentReturn {
    cc_tree!(consecuit_object,
        <div>
            <my_other_component />
            "blah blah"
        </div>
    )
}
```
*/
#[proc_macro]
pub fn cc_tree(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let CcTreeInput { cc_name, nodes } = parse_macro_input!(input as CcTreeInput);
    let tokens = nodes.build_top(cc_name);
    tokens.into()
}

struct CcTreeInput {
    cc_name: Ident,
    nodes: ConsecuitNodes,
}

impl Parse for CcTreeInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let cc_name = if input.peek2(Token![,]) {
            let cc_name: Ident = input.parse()?;
            let _comma: Token![,] = input.parse()?;
            cc_name
        } else {
            Ident::new("cc", Span::call_site())
        };
        let nodes = ConsecuitNodes::parse(input)?;
        Ok(Self { cc_name, nodes })
    }
}

struct ConsecuitNodes(Vec<ConsecuitNode>);

struct ConsecuitNode {
    func: ExprPath,
    props: ExprBlock,
    children: ConsecuitNodes,
    hole: bool,
}

impl Parse for ConsecuitNodes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parser_cfg = syn_rsx::ParserConfig::default();
        let parser = syn_rsx::Parser::new(parser_cfg);
        let nodes: Vec<Node> = parser.parse(input)?;
        let nodes: syn::Result<Vec<ConsecuitNode>> =
            nodes.into_iter().map(ConsecuitNode::try_from).collect();
        let nodes = nodes?;
        Ok(ConsecuitNodes(nodes))
    }
}

impl TryFrom<Node> for ConsecuitNode {
    type Error = syn::Error;
    fn try_from(node: Node) -> syn::Result<Self> {
        fn expr_into_text_node(expr: syn::Expr) -> ConsecuitNode {
            let func = parse_quote!(::consecuit_html::misc_components::text_node);
            let props = parse_quote!({#expr});
            ConsecuitNode {
                func,
                props,
                children: ConsecuitNodes(Vec::new()),
                hole: false,
            }
        }

        let Node {
            name,
            node_type,
            value,
            attributes,
            children,
        } = node;
        let res = match node_type {
            syn_rsx::NodeType::Element => {
                let name = name.unwrap();
                let func = if let syn_rsx::NodeName::Path(path) = &name {
                    Ok(path.clone())
                } else {
                    Err(syn::Error::new(name.span(), "Invalid component"))
                }?;

                let children: syn::Result<Vec<Self>> =
                    children.into_iter().map(ConsecuitNode::try_from).collect();
                let children = ConsecuitNodes(children?);

                let mut props = None;
                let mut hole = false;
                for attribute in attributes.iter() {
                    match attribute.node_type {
                        syn_rsx::NodeType::Block => {
                            if let syn::Expr::Block(block) = attribute.value.clone().unwrap() {
                                props = Some(block);
                            } else {
                                panic!()
                            }
                        }
                        syn_rsx::NodeType::Attribute => {
                            if attribute.name_as_string() == Some("HOLE".to_string()) {
                                hole = true;
                                if !children.0.is_empty() {
                                    return Err(syn::Error::new(
                                        attribute.name_span().unwrap(),
                                        "Only childless elements can be hole.",
                                    ));
                                }
                            }
                        }
                        _ => panic!(),
                    }
                }
                let props =
                    props.unwrap_or_else(|| parse_quote!({ ::std::default::Default::default() }));
                ConsecuitNode {
                    func,
                    props,
                    children,
                    hole,
                }
            }
            syn_rsx::NodeType::Attribute => panic!(),
            syn_rsx::NodeType::Text => expr_into_text_node(value.unwrap()),
            syn_rsx::NodeType::Comment => todo!(),
            syn_rsx::NodeType::Doctype => todo!(),
            syn_rsx::NodeType::Fragment => todo!(),
            syn_rsx::NodeType::Block => expr_into_text_node(value.unwrap()),
        };
        Ok(res)
    }
}

impl ConsecuitNodes {
    fn build_top(&self, cc_name: Ident) -> TokenStream {
        let tokens = self.build(cc_name.clone());
        quote! {
            #[allow(unused_braces)]
            #cc_name
            #tokens
        }
    }
    fn build(&self, cc_name: Ident) -> TokenStream {
        let tokens = self.0.iter().map(|n| {
            let ConsecuitNode {
                func,
                props,
                children,
                hole,
            } = n;
            let child_part = if children.0.is_empty() {
                if *hole {
                    quote! {
                        .hole_here()
                    }
                } else {
                    quote! {}
                }
            } else {
                let inner = children.build(cc_name.clone());
                quote! {
                    .child(|#cc_name| {
                        #cc_name
                        #inner
                    })
                }
            };
            quote! {
                .comp(#func, #props)#child_part
            }
        });
        quote! {
            #(#tokens)*
        }
    }
}
