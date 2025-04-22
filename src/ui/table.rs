use leptos::html::{
    Caption as HtmlCaption, Table as HtmlTable, Tbody as HtmlTbody, Td as HtmlTd,
    Tfoot as HtmlTfoot, Th as HtmlTh, Thead as HtmlThead, Tr as HtmlTr,
};
use leptos::prelude::*;
use crate::cn;

#[component]
#[allow(non_snake_case)]
pub fn Table(
    /// Optional extra classes to combine with defaults.
    #[prop(into, optional)]
    class: MaybeProp<String>,
    /// If you need a node ref to the `<table>` itself.
    #[prop(optional)]
    node_ref: NodeRef<HtmlTable>,
    /// Table contents (e.g. `<TableHeader>`, `<TableBody>`, etc.)
    children: Children,
) -> impl IntoView {
    view! {
        <div class="relative w-full overflow-auto">
            <table node_ref=node_ref class=cn!("w-full caption-bottom text-sm", class.get())>
                {children()}
            </table>
        </div>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn TableHeader(
    #[prop(into, optional)]
    class: MaybeProp<String>,
    #[prop(optional)]
    node_ref: NodeRef<HtmlThead>,
    children: Children,
) -> impl IntoView {
    view! {
        <thead node_ref=node_ref class=cn!("[&_tr]:border-b", class.get())>
            {children()}
        </thead>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn TableBody(
    #[prop(into, optional)]
    class: MaybeProp<String>,
    #[prop(optional)]
    node_ref: NodeRef<HtmlTbody>,
    children: Children,
) -> impl IntoView {
    view! {
        <tbody node_ref=node_ref class=cn!("[&_tr:last-child]:border-0", class.get())>
            {children()}
        </tbody>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn TableFooter(
    #[prop(into, optional)]
    class: MaybeProp<String>,
    #[prop(optional)]
    node_ref: NodeRef<HtmlTfoot>,
    children: Children,
) -> impl IntoView {
    view! {
        <tfoot
            node_ref=node_ref
            class=cn!("border-t bg-muted/50 font-medium [&>tr]:last:border-b-0", class.get())
        >
            {children()}
        </tfoot>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn TableRow(
    #[prop(into, optional)]
    class: MaybeProp<String>,
    #[prop(optional)]
    node_ref: NodeRef<HtmlTr>,
    children: Children,
) -> impl IntoView {
    view! {
        <tr
            node_ref=node_ref
            class=cn!(
                "border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted",
                class.get()
            )
        >
            {children()}
        </tr>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn TableHead(
    #[prop(into, optional)]
    class: MaybeProp<String>,
    #[prop(optional)]
    node_ref: NodeRef<HtmlTh>,
    children: Children,
) -> impl IntoView {
    view! {
        <th
            node_ref=node_ref
            class=cn!(
                "h-12 px-4 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0",
                class.get()
            )
        >
            {children()}
        </th>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn TableCell(
    #[prop(into, optional)]
    class: MaybeProp<String>,
    #[prop(optional)]
    node_ref: NodeRef<HtmlTd>,
    children: Children,
) -> impl IntoView {
    view! {
        <td
            node_ref=node_ref
            class=cn!("p-4 align-middle [&:has([role=checkbox])]:pr-0", class.get())
        >
            {children()}
        </td>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn TableCaption(
    #[prop(into, optional)]
    class: MaybeProp<String>,
    #[prop(optional)]
    node_ref: NodeRef<HtmlCaption>,
    children: Children,
) -> impl IntoView {
    view! {
        <caption node_ref=node_ref class=cn!("mt-4 text-sm text-muted-foreground", class.get())>
            {children()}
        </caption>
    }
}
