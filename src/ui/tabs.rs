use leptos::prelude::*;
use leptos_maybe_callback::MaybeCallback;
use leptos_node_ref::AnyNodeRef;
use crate::cn;
use crate::ui::radix_leptos_tabs::primitive as TabsPrimitive;

// 1) Our re-exported `<Tabs>`:
//    (Shadcn doesn't do extra styling for the root, so we just wrap RadixTabsRoot)
#[component]
#[allow(non_snake_case)]
pub fn Tabs(
    #[prop(optional)] value: Option<String>,
    #[prop(optional, into, default=String::new())]
    default_value: String,
    #[prop(optional, into)] on_value_change: MaybeCallback<String>,
    #[prop(optional, into)] orientation: MaybeProp<String>,
    #[prop(optional, into)] dir: MaybeProp<String>,
    #[prop(optional, into)] activation_mode: MaybeProp<String>,
    /// Additional classes if you want
    #[prop(optional, into)] class: MaybeProp<String>,
    /// If you want a node ref
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <TabsPrimitive::Root
            value=value
            default_value=default_value
            on_value_change=on_value_change
            orientation=orientation
            dir=dir
            activation_mode=activation_mode
            node_ref=node_ref
            attr:class=move || cn!(class.get().unwrap_or_default())
        >
            {children.with_value(|c| c())}
        </TabsPrimitive::Root>
    }
}

// 2) Our re-exported `<TabsList>` with the shadcn classes
#[component]
#[allow(non_snake_case)]
pub fn TabsList(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, default=true)]
    loop_: bool,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <TabsPrimitive::List
            loop_=loop_
            node_ref=node_ref
            attr:class=move || {
                cn!(
                    "inline-flex h-9 items-center justify-center rounded-lg bg-muted p-1 text-muted-foreground",
                class.get().unwrap_or_default(),
                )
            }
        >
            {children.with_value(|c| c())}
        </TabsPrimitive::List>
    }
}

// 3) Our re-exported `<TabsTrigger>` with shadcn classes
#[component]
#[allow(non_snake_case)]
pub fn TabsTrigger(
    #[prop(into)] value: String,
    #[prop(optional, default=false)] disabled: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <TabsPrimitive::Trigger
            value=value
            disabled=disabled
            node_ref=node_ref
            attr:class=move || {
                cn!(
                    "inline-flex items-center justify-center whitespace-nowrap rounded-md px-3 py-1 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 data-[state=active]:bg-background data-[state=active]:text-foreground data-[state=active]:shadow",
                class.get().unwrap_or_default()
                )
            }
        >
            {children.with_value(|c| c())}
        </TabsPrimitive::Trigger>
    }
}

// 4) Our re-exported `<TabsContent>` with shadcn classes
#[component]
#[allow(non_snake_case)]
pub fn TabsContent(
    #[prop(optional, into)] value: String,
    #[prop(optional, default=false)] force_mount: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    view! {
        <TabsPrimitive::Content
            value=value
            force_mount=force_mount
            node_ref=node_ref
            attr:class=move || {
                cn!(
                    "mt-2 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2",
                class.get().unwrap_or_default()
                )
            }
        >
            {children.with_value(|c| c())}
        </TabsPrimitive::Content>
    }
}
