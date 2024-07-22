use std::collections::BTreeMap;

use anchor_syn::idl::types::{IdlField, IdlTypeDefinition, IdlTypeDefinitionTy};

use heck::ToPascalCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{generate_fields, get_field_list_properties, StructOpts, ZeroCopy};

/// Generates an account state struct.
pub fn generate_account(
    defs: &[IdlTypeDefinition],
    account_name: &str,
    fields: &[IdlField],
    opts: StructOpts,
) -> TokenStream {
    let props = get_field_list_properties(defs, fields);

    let derive_default = if props.can_derive_default {
        quote! {
            #[derive(Default)]
        }
    } else {
        quote! {}
    };

    let zc_derive = if let Some(zero_copy) = opts.zero_copy {
        match zero_copy {
            ZeroCopy::Safe => quote! {
                #[zero_copy]
            },
            ZeroCopy::Unsafe => quote! {
                #[zero_copy(unsafe)]
            },
        }
    } else {
        quote! {}
    };

    let repr_derive = if let Some(repr) = opts.representation {
        match repr {
            crate::Representation::C => quote! {
                #[repr(C)]
            },
            crate::Representation::Transparent => quote! {
                #[repr(transparent)]
            },
            crate::Representation::Packed => quote! {
                #[repr(packed)]
            },
            crate::Representation::U8 => quote! {},
            crate::Representation::U64 => quote! {},
            crate::Representation::CPacked8 => quote! {
                #[repr(C, packed(8))]
            },
            crate::Representation::CPacked16 => quote! {
                #[repr(C, packed(16))]
            },
            crate::Representation::CAlign8 => quote! {
                #[repr(C, align(8))]
            },
            crate::Representation::CAlign16 => quote! {
                #[repr(C, align(16))]
            },
        }
    } else {
        quote! {}
    };

    let acct_derive = if opts.zero_copy.is_none() && opts.representation.is_none() {
        quote! {
            #[derive(AnchorSerialize, AnchorDeserialize, Clone)]
        }
    } else {
        quote! {}
    };

    let borsh_derive = if opts.can_borsh {
        // Add Clone if zero_copy is not set. (zero_copy already adds Clone)
        let add_clone = if opts.zero_copy.is_none() {
            quote! {
                #[derive(Clone)]
            }
        } else {
            quote! {}
        };

        quote! {
            #[derive(AnchorSerialize, AnchorDeserialize)]
            #add_clone
        }
    } else {
        quote! {}
    };

    // let doc = format!(" Account: {}", account_name);
    let struct_name = format_ident!("{}", account_name.to_pascal_case());
    let fields_rendered = generate_fields(fields);
    quote! {
        #zc_derive
        #repr_derive
        #acct_derive
        #derive_default
        #borsh_derive
        #[derive(Debug)]
        pub struct #struct_name {
            #fields_rendered
        }
    }
}

/// Generates account state structs.
pub fn generate_accounts(
    typedefs: &[IdlTypeDefinition],
    account_defs: &[IdlTypeDefinition],
    struct_opts: &BTreeMap<String, StructOpts>,
) -> TokenStream {
    let defined = account_defs.iter().map(|def| match &def.ty {
        IdlTypeDefinitionTy::Struct { fields } => {
            let opts = struct_opts.get(&def.name).cloned().unwrap_or_default();
            generate_account(typedefs, &def.name, fields, opts)
        }
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
