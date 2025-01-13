use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, DeriveInput};

#[proc_macro_derive(Entity)]
pub fn entity_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl entity::entity::Entity for #name {
            // Status

            fn get_current_hp(&self) -> &u32 {
                self.hp.get_level()
            }

            fn get_max_hp(&self) -> &u32 {
                self.hp.get_max()
            }

            // Interactions

            fn gain_hp(&mut self, amount: u32) -> Option<entity::health::HealthLog> {
                self.hp.increase(amount)
            }

            fn loose_hp(&mut self, amount: u32) {
                self.hp.decrease(amount);
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(EntityChangeMaxHealthCapacity)]
pub fn entity_change_max_health_capacity_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl entity::entity::EntityChangeMaxHealthCapacity for #name {
            fn set_max_hp(&mut self, new_max_hp: u32) {
                self.hp.set_max(new_max_hp);
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn entity(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            if let syn::Fields::Named(fields) = &mut struct_data.fields {
                fields.named.push(
                    syn::Field::parse_named
                        .parse2(quote! {hp: entity::health::Health})
                        .unwrap(),
                );
            }

            quote! {
                #ast
            }
            .into()
        }
        _ => panic!("`entity` has to be used with structs"),
    }
}
