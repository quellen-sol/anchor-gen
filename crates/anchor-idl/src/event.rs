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
  let mut enum_fields = vec![];
  let mut match_arms = vec![];

  let defs = events.iter().map(|event| {
    let name = format_ident!("{}", event.name.to_pascal_case());
    let to_idl_fields = event.fields.iter().map(|f| IdlField {
      docs: None,
      name: f.name.clone(),
      ty: f.ty.clone(),
    }).collect::<Vec<_>>();
    let fields_rendered = generate_fields(&to_idl_fields);

    enum_fields.push(quote! {
      #name(#name)
    });

    let leading_hex = sha256::digest(format!("event:{}", event.name));
    let leading_bytes = &hex::decode(leading_hex).unwrap()[..8];
    let leading_u64 = u64::from_le_bytes(leading_bytes.try_into().unwrap());

    match_arms.push(quote! {
      #leading_u64 => {
        let event = #name::try_from_slice(data)?;
        Ok(EventUnion::#name(event))
      }
    });
  
    quote! {
      #[derive(AnchorDeserialize, Debug, Clone)]
      pub struct #name {
        #fields_rendered
      }
    }
  }).collect::<Vec<_>>();

  quote! {
    #[derive(Debug, Clone)]
    pub enum EventUnion {
      #(#enum_fields),*
    }

    impl EventUnion {
      pub fn try_from_slice(data: &[u8]) -> Result<Self> {
        let leading_u64 = u64::from_le_bytes(data[0..8].try_into()?);
        match leading_u64 {
          #(#match_arms),*,
          _ => Err(anyhow::anyhow!("Invalid event")),
        }
      }
    }

    #(#defs)*
  }
}