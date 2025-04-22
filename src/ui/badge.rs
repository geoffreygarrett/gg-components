use leptos::{prelude::*, html};
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(class = "inline-flex items-center rounded-md border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2")]
pub struct BadgeStyles {
    variant: BadgeVariant,
}

#[derive(TwVariant)]
pub enum BadgeVariant {
    #[tw(default, class = "border-transparent bg-primary text-primary-foreground shadow hover:bg-primary/80")]
    Default,
    #[tw(class = "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80")]
    Secondary,
    #[tw(class = "border-transparent bg-destructive text-destructive-foreground shadow hover:bg-destructive/80")]
    Destructive,
    #[tw(class = "text-foreground")]
    Outline,
}

impl From<&str> for BadgeVariant {
    fn from(s: &str) -> Self {
        match s {
            "secondary" => BadgeVariant::Secondary,
            "destructive" => BadgeVariant::Destructive,
            "outline" => BadgeVariant::Outline,
            _ => BadgeVariant::Default,
        }
    }
}

impl From<String> for BadgeVariant {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

#[component]
pub fn Badge(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] variant: BadgeVariant,
    #[prop(optional, into)] node_ref: NodeRef<html::Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=BadgeStyles::builder()
                .variant(variant)
                .with_class(class.get().unwrap_or_default())
        >
            {children()}
        </div>
    }
}
