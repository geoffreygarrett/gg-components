// use leptos::context::Provider;
// use leptos::html;
// use leptos::prelude::*;
// use leptos_node_ref::AnyNodeRef;
// use radix_leptos_primitive::Primitive;
// use radix_leptos_id::use_id;
// use leptos_hook_form::controller::{Controller, UseControllerReturn};
// use leptos_hook_form::path::{FieldPath, FormState};
// use leptos_hook_form::types::{FieldValues, UseFormReturn};
// use crate::cn;
// use crate::components::ui::Label;
//
// // Form field context
// #[derive(Clone)]
// pub struct FormFieldContextValue<T: FieldValues> {
//     name: FieldPath<T>,
// }
//
// // Form item context
// #[derive(Clone)]
// pub struct FormItemContextValue {
//     id: String,
// }
//
// // Form field state structure
// #[derive(Clone)]
// pub struct FormFieldState {
//     pub id: String,
//     pub name: String,
//     pub form_item_id: String,
//     pub form_description_id: String,
//     pub form_message_id: String,
//     pub error: Option<FieldError>,
// }
//
// // Field error structure (assuming this is part of your project)
// #[derive(Clone)]
// pub struct FieldError {
//     pub message: String,
// }
//
// #[component]
// pub fn FormField<T, F, C>(
//     #[prop(into)] children: Children,
//     #[prop(into)] name: FieldPath<T>,
//     #[prop(into, optional)] default_value: Option<T>,
//     #[prop(into, optional)] disabled: bool,
//     render: F,
// ) -> impl IntoView
// where
//     T: FieldValues,
//     F: Fn(UseControllerReturn<T>) -> C + 'static + Send + Sync,
//     C: IntoView + 'static,
// {
//     let context = FormFieldContextValue { name: name.clone() };
//
//     view! {
//         <Provider value=context>
//             <Controller name=name default_value=default_value disabled=disabled render=render />
//             {children()}
//         </Provider>
//     }
// }
//
// // Context provider setup
// #[derive(Clone)]
// pub struct FormContext<T: FieldValues> {
//     pub form: UseFormReturn<T>,
// }
//
// // Hook to access form context
// pub fn use_form_context<T>() -> FormContext<T>
// where
//     T: FieldValues + Clone + 'static,
// {
//     use_context::<FormContext<T>>()
//         .expect("use_form_context must be used within a FormProvider")
// }
//
//
// pub fn use_form_field<T: FieldValues + 'static>() -> FormFieldState {
//     let field_context = use_context::<FormFieldContextValue<T>>()
//         .expect("use_form_field should be used within <FormField>");
//     let item_context = use_context::<FormItemContextValue>()
//         .expect("use_form_field should be used within <FormItem>");
//
//     let form = use_form_context::<T>();
//     let field_state = form.get_field_state(&field_context.name);
//
//     FormFieldState {
//         id: item_context.id.clone(),
//         name: field_context.name.to_string(),
//         form_item_id: format!("{}-form-item", item_context.id),
//         form_description_id: format!("{}-form-item-description", item_context.id),
//         form_message_id: format!("{}-form-item-message", item_context.id),
//         error: field_state.error,
//     }
// }
//
// #[component]
// pub fn FormItem(
//     #[prop(into)] children: Children,
//     #[prop(optional)] class: Option<String>,
//     #[prop(optional)] node_ref: NodeRef<html::Div>,
// ) -> impl IntoView {
//     let id = use_id(None);
//     provide_context(FormItemContextValue { id: id.get() });
//
//     view! {
//         <div node_ref=node_ref class=cn!("space-y-2", class)>
//             {children()}
//         </div>
//     }
// }
//
// #[component]
// pub fn FormLabel(
//     #[prop(into)] children: ChildrenFn,
//     #[prop(optional)] class: Option<String>,
//     #[prop(optional)] node_ref: NodeRef<html::Label>,
// ) -> impl IntoView {
//     let children = StoredValue::new(children);
//     let field_state = use_form_field();
//
//     view! {
//         <Label
//             node_ref=node_ref
//             class=cn!((field_state.error.is_some(), "text-destructive"),
//                 class)
//             {..}
//             for=field_state.form_item_id
//         >
//             {children.with_value(|c| c())}
//         </Label>
//     }
// }
//
//
// // FORM CONTROL COMPONENT
// #[component]
// pub fn FormControl(
//     #[prop(into)] children: Children,
//     #[prop(optional)] class: Option<String>,
//     #[prop(optional)] node_ref: NodeRef<html::Div>,
// ) -> impl IntoView {
//     let field_state = use_form_field();
//     let has_error = move || field_state.error.is_some();
//
//     let described_by = move || {
//         format!(
//             "{} {}",
//             field_state.form_description_id,
//             field_state.form_message_id
//         )
//     };
//
//     view! {
//         <div
//             node_ref=node_ref
//             id=field_state.form_item_id
//             aria-describedby=described_by
//             aria-invalid=move || has_error().to_string()
//             class=class
//         >
//             {children()}
//         </div>
//     }
// }
//
// // FORM DESCRIPTION COMPONENT
// #[component]
// pub fn FormDescription(
//     #[prop(into)] children: Children,
//     #[prop(optional)] class: Option<String>,
//     #[prop(optional)] node_ref: NodeRef<html::P>,
// ) -> impl IntoView {
//     let field_state = use_form_field();
//
//     view! {
//         <p
//             node_ref=node_ref
//             id=field_state.form_description_id
//             class=cn!("text-sm text-muted-foreground", class)
//         >
//             {children()}
//         </p>
//     }
// }
//
// // FORM MESSAGE COMPONENT
// #[component]
// pub fn FormMessage(
//     #[prop(optional)] children: Option<ChildrenFn>,
//     #[prop(optional)] class: Option<String>,
//     #[prop(optional)] node_ref: NodeRef<html::P>,
// ) -> impl IntoView {
//     let children = StoredValue::new(children);
//     let field_state = use_form_field();
//     let error_message = Memo::new(move |_| {
//         field_state.error
//             .as_ref()
//             .map(|e| e.message.clone())
//             .unwrap_or_default()
//     });
//
//     let show_message = move || {
//         field_state.error.is_some() || children.with_value(|c| c.is_some())
//     };
//
//     view! {
//         <Show when=show_message>
//             <p
//                 node_ref=node_ref
//                 id=field_state.form_message_id
//                 class=cn!("text-sm font-medium text-destructive", class)
//             >
//                 {move || {
//                     children
//                         .with_value(|c| {
//                             c.as_ref()
//                                 .map(|c| c().into_any())
//                                 .unwrap_or_else(|| error_message.get().into_any())
//                         })
//                 }}
//             //
//             </p>
//         </Show>
//     }
// }
//
// #[component]
// pub fn FormProvider<T>(
//     form: UseFormReturn<T>,
//     children: Children,
// ) -> impl IntoView
// where
//     T: FieldValues + Clone + 'static,
// {
//     view! { <Provider value=FormContext { form }>{children()}</Provider> }
// }
//
// // // FORM COMPONENT (WRAPPER)
// // #[component]
// // pub fn Form<T>(
// //     #[prop(into)] children: Children,
// //     form: FormState<T>,
// //     #[prop(optional)] class: Option<String>,
// //     #[prop(into, optional)] as_child: bool,
// //     #[prop(into, optional)] node_ref: AnyNodeRef,
// // ) -> impl IntoView
// // where
// //     T: FieldValues + 'static,
// // {
// //     let form = use_form::<T>(FormConfig::default());
// //
// //     view! {
// //         <FormProvider form=form>
// //             <Primitive
// //                 element={html::form}
// //                 node_ref={node_ref}
// //                 {..}
// //                 class=class
// //                 on:submit=move |ev| form.handle_submit(ev)
// //             >
// //                 {children()}
// //             </Primitive>
// //         </FormProvider>
// //     }
// // }