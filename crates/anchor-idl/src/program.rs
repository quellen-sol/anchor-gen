use std::{
    collections::{BTreeMap, HashSet},
    env, fs,
    path::PathBuf,
};

use darling::{util::PathList, FromMeta};
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    generate_accounts, generate_events, generate_ix_deser_structs, generate_typedefs, GEN_VERSION,
};

#[derive(Default, FromMeta)]
pub struct GeneratorOptions {
    /// Path to the IDL.
    pub idl_path: String,
    /// List of zero copy structs.
    pub zero_copy: Option<PathList>,
    /// List of anchor legacy zero copy structs.
    pub zero_copy_unsafe: Option<PathList>,
    /// List of `repr(C)` structs.
    pub c_representation: Option<PathList>,
    /// List of `repr(C, packed(8))` structs with C repr,
    pub c_packed_8_representation: Option<PathList>,
    /// List of `repr(C, packed(16))` structs with C repr,
    pub c_packed_16_representation: Option<PathList>,
    /// List of `repr(C, align(8))` structs with C repr,
    pub c_align_8_representation: Option<PathList>,
    /// List of `repr(C, align(16))` structs with C repr,
    pub c_align_16_representation: Option<PathList>,
    /// List of `repr(transparent)` structs.
    pub transparent_representation: Option<PathList>,
    /// List of `repr(packed)` structs.
    pub packed_representation: Option<PathList>,
    /// List of `repr(u8)` structs.
    pub u8_representation: Option<PathList>,
    /// List of `repr(u64)` structs.
    pub u64_representation: Option<PathList>,
    /// List of `BorshDeserialize` structs.
    pub borsh: Option<PathList>,
}

fn path_list_to_string(list: Option<&PathList>) -> HashSet<String> {
    list.map(|el| {
        el.iter()
            .map(|el| el.get_ident().unwrap().to_string())
            .collect()
    })
    .unwrap_or_default()
}

impl GeneratorOptions {
    pub fn to_generator(&self) -> Generator {
        let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = PathBuf::from(cargo_manifest_dir).join(&self.idl_path);
        let idl_contents = fs::read_to_string(path).unwrap();
        let idl: anchor_syn::idl::types::Idl = serde_json::from_str(&idl_contents).unwrap();

        let zero_copy_safe = path_list_to_string(self.zero_copy.as_ref());

        let zero_copy_unsafe = path_list_to_string(self.zero_copy_unsafe.as_ref());

        let c_repr = path_list_to_string(self.c_representation.as_ref());

        let c_p8_repr = path_list_to_string(self.c_packed_8_representation.as_ref());

        let c_p16_repr = path_list_to_string(self.c_packed_16_representation.as_ref());

        let c_a8_repr = path_list_to_string(self.c_align_8_representation.as_ref());

        let c_a16_repr = path_list_to_string(self.c_align_16_representation.as_ref());

        let transparent_repr = path_list_to_string(self.transparent_representation.as_ref());

        let packed_repr = path_list_to_string(self.packed_representation.as_ref());

        let u8_repr = path_list_to_string(self.u8_representation.as_ref());

        let u64_repr = path_list_to_string(self.u64_representation.as_ref());

        let borsh_des = path_list_to_string(self.borsh.as_ref());

        let repr = c_repr
            .union(&c_p8_repr)
            .cloned()
            .collect::<HashSet<_>>()
            .union(&c_p16_repr)
            .cloned()
            .collect::<HashSet<_>>()
            .union(&c_a8_repr)
            .cloned()
            .collect::<HashSet<_>>()
            .union(&c_a16_repr)
            .cloned()
            .collect::<HashSet<_>>()
            .union(&transparent_repr)
            .cloned()
            .collect::<HashSet<_>>()
            .union(&packed_repr)
            .cloned()
            .collect::<HashSet<_>>()
            .union(&u8_repr)
            .cloned()
            .collect::<HashSet<_>>()
            .union(&u64_repr)
            .cloned()
            .collect::<HashSet<_>>();

        let zero_copy = zero_copy_safe
            .union(&zero_copy_unsafe)
            .cloned()
            .collect::<HashSet<_>>();

        let mut struct_opts: BTreeMap<String, StructOpts> = BTreeMap::new();
        let all_structs: HashSet<&String> = zero_copy.union(&repr).collect::<HashSet<_>>();

        all_structs.into_iter().for_each(|name| {
            let is_c_repr = c_repr.contains(name);
            let is_c_p8_repr = c_p8_repr.contains(name);
            let is_c_p16_repr = c_p16_repr.contains(name);
            let is_c_a8_repr = c_a8_repr.contains(name);
            let is_c_a16_repr = c_a16_repr.contains(name);

            let is_transparent_repr = transparent_repr.contains(name);
            let is_packed_repr = packed_repr.contains(name);
            let is_u8_repr = u8_repr.contains(name);
            let is_u64_repr = u64_repr.contains(name);

            let representation = match (
                is_c_repr,
                is_transparent_repr,
                is_packed_repr,
                is_u8_repr,
                is_u64_repr,
                is_c_p8_repr,
                is_c_p16_repr,
                is_c_a8_repr,
                is_c_a16_repr,
            ) {
                (true, false, false, false, false, false, false, false, false) => {
                    Some(Representation::C)
                }
                (false, true, false, false, false, false, false, false, false) => {
                    Some(Representation::Transparent)
                }
                (false, false, true, false, false, false, false, false, false) => {
                    Some(Representation::Packed)
                }
                (false, false, false, true, false, false, false, false, false) => {
                    Some(Representation::U8)
                }
                (false, false, false, false, true, false, false, false, false) => {
                    Some(Representation::U64)
                }
                (false, false, false, false, false, true, false, false, false) => {
                    Some(Representation::CPacked8)
                }
                (false, false, false, false, false, false, true, false, false) => {
                    Some(Representation::CPacked16)
                }
                (false, false, false, false, false, false, false, true, false) => {
                    Some(Representation::CAlign8)
                }
                (false, false, false, false, false, false, false, false, true) => {
                    Some(Representation::CAlign16)
                }
                (false, false, false, false, false, false, false, false, false) => None,
                _ => panic!("type {name} cannot have many representations"),
            };

            let is_zero_copy_safe = zero_copy_safe.contains(name);
            let is_zero_copy_unsafe = zero_copy_unsafe.contains(name);

            let zero_copy = match (is_zero_copy_safe, is_zero_copy_unsafe) {
                (true, true) => panic!("cant be safe and unsafe zero copy at the same time"),
                (true, false) => Some(ZeroCopy::Safe),
                (false, true) => Some(ZeroCopy::Unsafe),
                (false, false) => None,
            };

            let can_borsh = borsh_des.contains(name);

            struct_opts.insert(
                name.to_string(),
                StructOpts {
                    representation,
                    zero_copy,
                    can_borsh,
                },
            );
        });

        Generator { idl, struct_opts }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct StructOpts {
    pub representation: Option<Representation>,
    pub zero_copy: Option<ZeroCopy>,
    pub can_borsh: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum ZeroCopy {
    Unsafe,
    Safe,
}

#[derive(Clone, Copy, Debug)]
pub enum Representation {
    C,
    Transparent,
    Packed,
    U8,
    U64,
    CPacked8,
    CPacked16,
    CAlign8,
    CAlign16,
}
pub struct Generator {
    pub idl: anchor_syn::idl::types::Idl,
    pub struct_opts: BTreeMap<String, StructOpts>,
}

impl Generator {
    pub fn generate_cpi_interface(&self) -> TokenStream {
        let idl = &self.idl;

        let accounts = generate_accounts(&idl.types, &idl.accounts, &self.struct_opts);
        let typedefs = generate_typedefs(&idl.types, &self.struct_opts);
        // let ix_handlers = generate_ix_handlers(&idl.instructions);
        // let ix_structs = generate_ix_structs(&idl.instructions);
        let ix_structs = generate_ix_deser_structs(&idl.instructions);
        let events = generate_events(&idl.events);

        let _docs = format!(
        " Anchor CPI crate generated from {} v{} using [anchor-gen](https://crates.io/crates/anchor-gen) v{}.",
        &idl.name,
        &idl.version,
        &GEN_VERSION.unwrap_or("unknown")
    );

        quote! {
            use anchor_lang::prelude::*;
            use anyhow::Result;

            #typedefs
            #ix_structs
            #accounts
            #events
        }
    }
}
