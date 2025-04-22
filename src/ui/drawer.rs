// use std::sync::Arc;
// use leptos::{
//     component,
//     html,
//     prelude::*,
//     attribute_interceptor::AttributeInterceptor,
// };
// use leptos_node_ref::prelude::{AnyNodeRef, IntoAnyNodeRef};
//
// pub use vaul_leptos::primitive as DrawerPrimitive;
// pub use DrawerPrimitive::Root as Drawer;
// pub use DrawerPrimitive::Trigger as DrawerTrigger;
// pub use DrawerPrimitive::Portal as DrawerPortal;
// pub use DrawerPrimitive::Close as DrawerClose;
//
// use crate::cn;
//
// #[component(transparent)]
// #[allow(non_snake_case)]
// pub fn DrawerOverlay(
//     #[prop(into, optional)] class: MaybeProp<String>,
//     #[prop(into, optional)] node_ref: AnyNodeRef,
// ) -> impl IntoView {
//     view! {
//         <DrawerPrimitive::Overlay
//             node_ref=node_ref
//             attr:class=move || cn!("fixed inset-0 z-50 bg-black/80", class.get())
//         />
//     }
// }
//
// #[component(transparent)]
// #[allow(non_snake_case)]
// pub fn DrawerContent(
//     children: ChildrenFn,
//     #[prop(into, optional)] class: MaybeProp<String>,
//     #[prop(into, optional)] node_ref: AnyNodeRef,
// ) -> impl IntoView {
//     let children = StoredValue::new(children);
//     view! {
//         <DrawerPortal>
//             <DrawerOverlay />
//             <DrawerPrimitive::Content
//                 node_ref=node_ref
//                 attr:class=move || {
//                     cn!(
//                         "fixed inset-x-0 bottom-0 z-50 mt-24 flex h-auto flex-col rounded-t-[10px] border bg-background",
//                         class.get()
//                     )
//                 }
//             >
//                 <div class="mx-auto mt-4 h-2 w-[100px] rounded-full bg-muted"></div>
//                 {children.with_value(|children| children())}
//             </DrawerPrimitive::Content>
//         </DrawerPortal>
//     }
// }
//
// #[component]
// #[allow(non_snake_case)]
// pub fn DrawerHeader(
//     children: Children,
//     #[prop(into, optional)] class: MaybeProp<String>,
// ) -> impl IntoView {
//     view! {
//         <div
//             class=move || cn!("grid gap-1.5 p-4 text-center sm:text-left", class.get())
//         >
//             {children()}
//         </div>
//     }
// }
//
// #[component]
// #[allow(non_snake_case)]
// pub fn DrawerFooter(
//     children: Children,
//     #[prop(into, optional)] class: MaybeProp<String>,
// ) -> impl IntoView {
//     view! {
//         <div
//             class=move || cn!("mt-auto flex flex-col gap-2 p-4", class.get())
//         >
//             {children()}
//         </div>
//     }
// }
//
// #[component(transparent)]
// #[allow(non_snake_case)]
// pub fn DrawerTitle(
//     children: ChildrenFn,
//     #[prop(into, optional)] class: MaybeProp<String>,
//     #[prop(into, optional)] node_ref: AnyNodeRef,
// ) -> impl IntoView {
//     let children = StoredValue::new(children);
//     view! {
//         <DrawerPrimitive::Title
//             node_ref=node_ref
//             attr:class=move || {
//                 cn!(
//                     "text-lg font-semibold leading-none tracking-tight",
//                     class.get()
//                 )
//             }
//         >
//             {children.with_value(|children| children())}
//         </DrawerPrimitive::Title>
//     }
// }
//
// #[component(transparent)]
// #[allow(non_snake_case)]
// pub fn DrawerDescription(
//     children: ChildrenFn,
//     #[prop(into, optional)] class: MaybeProp<String>,
//     #[prop(into, optional)] node_ref: AnyNodeRef,
// ) -> impl IntoView {
//     let children = StoredValue::new(children);
//     view! {
//         <DrawerPrimitive::Description
//             node_ref=node_ref
//             attr:class=move || cn!("text-sm text-muted-foreground", class.get())
//         >
//             {children.with_value(|children| children())}
//         </DrawerPrimitive::Description>
//     }
// }
//
// // Example usage:
// #[component]
// pub fn Example() -> impl IntoView {
//     view! {
//         <Drawer>
//             <DrawerTrigger>
//                 <button>"Open Drawer"</button>
//             </DrawerTrigger>
//             <DrawerContent>
//                 <DrawerHeader>
//                     <DrawerTitle>"Title"</DrawerTitle>
//                     <DrawerDescription>"Description"</DrawerDescription>
//                 </DrawerHeader>
//                 <div class="p-4">"Content"</div>
//                 <DrawerFooter>
//                     <DrawerClose>
//                         <button>"Close"</button>
//                     </DrawerClose>
//                 </DrawerFooter>
//             </DrawerContent>
//         </Drawer>
//     }
// }