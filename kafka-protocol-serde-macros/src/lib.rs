extern crate proc_macro;

mod ser_struct;
use ser_struct::impl_serialize_for_struct;

mod ser_enum;
use ser_enum::impl_serialize_for_res_enum;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(KafkaDeserialize)]
pub fn derive_kafka_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let struct_data = match &input.data {
        syn::Data::Struct(struct_data) => struct_data,
        _ => todo!()
    };

    let fields = struct_data.fields.iter().map(|field| {
        let field_name = &field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        quote! {
            #field_name: <#field_type>::kafka_deserialize(reader)?
        }
    });

    let expanded = quote! {
        impl KafkaDeserialize for #name {
            fn kafka_deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
                let deserialized = Self {
                    #(#fields),*
                };

                Ok(deserialized)
            }
        }
    };

    TokenStream::from(expanded)
}


#[proc_macro_derive(KafkaSerialize)]
pub fn derive_kafka_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let data = input.data;
    match &data {
        syn::Data::Struct(struct_data) => impl_serialize_for_struct(name, struct_data),
        syn::Data::Enum(enum_data) => {
            if name.to_string().contains("Response") {
                return impl_serialize_for_res_enum(name, enum_data);
            }

            panic!("Unsupported enum")
        },
        _ => todo!()
    }
}

