use leptos::html;
use leptos::prelude::*;
use radix_leptos_separator::{Separator as SeparatorPrimitive, Orientation as SeparatorPrimitiveOrientation};
use tailwind_fuse::*;

impl From<SeparatorOrientation> for SeparatorPrimitiveOrientation {
    fn from(value: SeparatorOrientation) -> Self {
        match value {
            SeparatorOrientation::Horizontal => SeparatorPrimitiveOrientation::Horizontal,
            SeparatorOrientation::Vertical => SeparatorPrimitiveOrientation::Vertical,
        }
    }
}

impl From<SeparatorOrientation> for String {
    fn from(value: SeparatorOrientation) -> Self {
        match value {
            SeparatorOrientation::Horizontal => "horizontal".into(),
            SeparatorOrientation::Vertical => "vertical".into(),
        }
    }
}

#[derive(TwVariant, PartialEq)]
pub enum SeparatorOrientation {
    #[tw(default, class = "h-[1px] w-full")]
    Horizontal,
    #[tw(class = "h-full w-[1px]")]
    Vertical,
}

#[derive(TwClass, PartialEq)]
#[tw(class = "shrink-0 bg-border")]
pub struct SeparatorStyle {
    orientation: SeparatorOrientation,
}

#[component]
#[allow(non_snake_case)]
pub fn Separator(
    /// Additional classes for customizing
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Orientation of the separator (horizontal or vertical)
    #[prop(into, optional, default = SeparatorOrientation::Horizontal)]
    orientation: SeparatorOrientation,
    /// Whether the separator is purely decorative
    #[prop(optional, default = true)] decorative: bool,
    /// A `NodeRef` if you need direct DOM access
    #[prop(optional, into)] node_ref: NodeRef<html::Div>,
) -> impl IntoView {
    // Combine base + variant classes
    let style = Signal::derive(move || {
        SeparatorStyle::builder()
            .orientation(orientation.clone())
            .with_class(class.get().unwrap_or_default())
    });

    view! {
        <SeparatorPrimitive
            node_ref=node_ref
            // Pass orientation to the underlying primitive if needed
            // some primitives expect a string, so we do:
            orientation=match orientation {
                SeparatorOrientation::Horizontal => SeparatorPrimitiveOrientation::Horizontal,
                SeparatorOrientation::Vertical => SeparatorPrimitiveOrientation::Vertical,
            }
            // Pass `decorative` to the underlying primitive
            decorative=decorative
            // Merge in the final Tailwind classes
            class=style
        />
    }
}