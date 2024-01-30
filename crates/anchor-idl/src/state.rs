use anchor_syn::idl::types::{IdlField, IdlTypeDefinition, IdlTypeDefinitionTy};
use heck::ToPascalCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{generate_fields, get_field_list_properties};

/// Generates an account state struct.
pub fn generate_account(
    defs: &[IdlTypeDefinition],
    account_name: &str,
    fields: &[IdlField],
) -> TokenStream {
    let _props = get_field_list_properties(defs, fields);

    // let doc = format!(" Account: {}", account_name);
    let struct_name = format_ident!("{}", account_name.to_pascal_case());
    let fields_rendered = generate_fields(fields);
    quote! {
        // #derive_account
        // // #[doc = #doc]
        // #derive_copy
        // #derive_default
        #[derive(AnchorDeserialize, Clone, Debug)]
        pub struct #struct_name {
            #fields_rendered
        }
    }
}

/// Generates account state structs.
pub fn generate_accounts(
    typedefs: &[IdlTypeDefinition],
    account_defs: &[IdlTypeDefinition],
) -> TokenStream {
    let defined = account_defs.iter().map(|def| match &def.ty {
        IdlTypeDefinitionTy::Struct { fields } => generate_account(typedefs, &def.name, fields),
        IdlTypeDefinitionTy::Enum { .. } => {
            panic!("unexpected enum account");
        }
        IdlTypeDefinitionTy::Alias { .. } => {
            panic!("unexpected alias account");
        }
    });
    quote! {
        #(#defined)*
    }
}
