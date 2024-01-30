use anchor_syn::idl::types::IdlInstruction;
use heck::{ToPascalCase, ToSnakeCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

pub fn format_ix_name(ix: &IdlInstruction) -> (Ident, Ident) {
    let ix_name = format_ident!("{}", ix.name.to_pascal_case());
    let ix_name_with_suffix = format_ident!("{}Ix", ix.name.to_pascal_case());
    (ix_name, ix_name_with_suffix)
}

pub fn generate_ix_deser_structs(ixs: &[IdlInstruction]) -> TokenStream {
    let mut enum_fields = vec![];
    let mut match_arms = vec![];
    let struct_defs = ixs
        .iter()
        .map(|ix| {
            let (ix_without_suffix, ix_name_with_suffix) = format_ix_name(ix);

            let args = ix
                .args
                .iter()
                .map(|arg| {
                    let name = format_ident!("{}", arg.name.to_snake_case());
                    let type_name = crate::ty_to_rust_type(&arg.ty);
                    let stream = type_name.parse::<TokenStream>().unwrap();
                    quote! {
                        pub #name: #stream
                    }
                })
                .collect::<Vec<_>>();

            enum_fields.push(quote! {
                #ix_without_suffix(#ix_name_with_suffix)
            });

            let leading_hex = sha256::digest(format!("global:{}", ix.name.to_snake_case()));
            let leading_bytes = &hex::decode(leading_hex).unwrap()[..8];
            let leading_u64 = u64::from_le_bytes(leading_bytes.try_into().unwrap());

            match_arms.push(quote! {
                #leading_u64 => {
                    let ix = #ix_name_with_suffix::try_from_slice(data)?;
                    Ok(InstructionUnion::#ix_without_suffix(ix))
                }
            });

            quote! {
                #[derive(AnchorDeserialize, Clone, Debug)]
                pub struct #ix_name_with_suffix {
                    #(#args),*
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        #[derive(Clone, Debug)]
        pub enum InstructionUnion {
            #(#enum_fields),*
        }

        impl InstructionUnion {
            pub fn try_from_slice(data: &[u8]) -> Result<Self> {
                let hex_enc = u64::from_le_bytes(data.try_into()?);
                match hex_enc {
                    #(#match_arms),*,
                    _ => Err(anyhow::anyhow!("unknown instruction")),
                }
            }
        }

        #(#struct_defs)*
    }
}

/// Generates a single instruction handler.
pub fn generate_ix_handler(ix: &IdlInstruction) -> TokenStream {
    let ix_name = format_ident!("{}", ix.name.to_snake_case());
    let accounts_name = format_ident!("{}", ix.name.to_pascal_case());

    let args = ix
        .args
        .iter()
        .map(|arg| {
            let name = format_ident!("_{}", arg.name.to_snake_case());
            let type_name = crate::ty_to_rust_type(&arg.ty);
            let stream: proc_macro2::TokenStream = type_name.parse().unwrap();
            quote! {
                #name: #stream
            }
        })
        .collect::<Vec<_>>();

    if cfg!(feature = "compat-program-result") {
        quote! {
            pub fn #ix_name(
                _ctx: Context<#accounts_name>,
                #(#args),*
            ) -> ProgramResult {
                unimplemented!("This program is a wrapper for CPI.")
            }
        }
    } else {
        quote! {
            pub fn #ix_name(
                _ctx: Context<#accounts_name>,
                #(#args),*
            ) -> Result<()> {
                unimplemented!("This program is a wrapper for CPI.")
            }
        }
    }
}

/// Generates instruction context structs.
pub fn generate_ix_structs(ixs: &[IdlInstruction]) -> TokenStream {
    let defs = ixs.iter().map(|ix| {
        let accounts_name = format_ident!("{}", ix.name.to_pascal_case());

        let (all_structs, all_fields) =
            crate::generate_account_fields(&ix.name.to_pascal_case(), &ix.accounts);

        quote! {
            #all_structs

            #[derive(Accounts)]
            pub struct #accounts_name<'info> {
                #all_fields
            }
        }
    });
    quote! {
        #(#defs)*
    }
}

/// Generates all instruction handlers.
pub fn generate_ix_handlers(ixs: &[IdlInstruction]) -> TokenStream {
    let streams = ixs.iter().map(generate_ix_handler);
    quote! {
        #(#streams)*
    }
}
