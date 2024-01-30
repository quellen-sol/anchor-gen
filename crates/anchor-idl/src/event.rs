use anchor_syn::idl::types::{IdlEvent, IdlField};
use heck::ToPascalCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::generate_fields;

pub fn generate_events(events: &Option<Vec<IdlEvent>>) -> TokenStream {
  if events.is_none() {
    return quote! {};
  }

  let events = events.as_ref().unwrap();

  let defs = events.iter().map(|event| {
    let name = format_ident!("{}", event.name.to_pascal_case());
    let to_idl_fields = event.fields.iter().map(|f| IdlField {
      docs: None,
      name: f.name.clone(),
      ty: f.ty.clone(),
    }).collect::<Vec<_>>();
    let fields_rendered = generate_fields(&to_idl_fields);

    quote! {
      #[derive(Debug, Clone)]
      pub struct #name {
        #fields_rendered
      }
    }
  }).collect::<Vec<_>>();

  quote! {
    #(#defs)*
  }
}