use leptos::html;
use leptos::prelude::*;
use tailwind_fuse::*;
use crate::cn;
// For TwClass/TwVariant; adjust imports as needed

#[derive(TwClass, PartialEq)]
#[tw(
    // Base alert styles (similar to the `cva` usage in the React version)
    class = "relative w-full rounded-lg border p-4 \
             [&>svg~*]:pl-7 [&>svg+div]:translate-y-[-3px] \
             [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 \
             [&>svg]:text-foreground"
)]
pub struct AlertStyles {
    variant: AlertVariant,
}

#[derive(TwVariant, PartialEq)]
pub enum AlertVariant {
    #[tw(
        // Default variant
        default,
        class = "bg-background text-foreground"
    )]
    Default,
    #[tw(
        // Destructive variant
        class = "border-destructive/50 text-destructive \
                 dark:border-destructive [&>svg]:text-destructive"
    )]
    Destructive,
}

/// For easy string-based assignment, if you like:
impl From<&str> for AlertVariant {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "destructive" => AlertVariant::Destructive,
            _ => AlertVariant::Default,
        }
    }
}


#[component]
#[allow(non_snake_case)]
pub fn Alert(
    /// Which variant to use (Default or Destructive)
    #[prop(optional, into)] variant: AlertVariant,
    /// Additional classes that you might want to merge in
    #[prop(optional, into)] class: MaybeProp<String>,
    /// For capturing or manipulating the DOM node directly
    #[prop(optional, into)] node_ref: NodeRef<html::Div>,
    /// Children inside the alert (e.g. icon, title, description, etc.)
    children: Children,
) -> impl IntoView {
    let styles = Signal::derive(move || {
        AlertStyles::builder()
            .variant(variant)
            .with_class(class.get().unwrap_or_default())
    });

    view! {
        <div
            node_ref=node_ref
            // Mirror role="alert" from the React version
            role="alert"
            class=move || styles.get()
        >
            {children()}
        </div>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn AlertTitle(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] node_ref: NodeRef<html::H5>,
    children: Children,
) -> impl IntoView {
    view! {
        <h5
            node_ref=node_ref
            class=cn!("mb-1 font-medium leading-none tracking-tight", class.get())
        >
            {children()}
        </h5>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn AlertDescription(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] node_ref: NodeRef<html::Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=cn!("text-sm [&_p]:leading-relaxed", class.get())>
            {children()}
        </div>
    }
}
