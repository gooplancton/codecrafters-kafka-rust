use proc_macro::TokenStream;
use proc_macro2::Ident;
use syn::DataStruct;
use quote::quote;

pub fn impl_serialize_for_struct(name: &Ident, struct_data: &DataStruct) -> TokenStream {
    let byte_lens = struct_data.fields.iter().map(|field| {
        let field_name = &field.ident.as_ref().unwrap();
        quote! { self.#field_name.kafka_byte_len() }
    });

    let buf_writes = struct_data.fields.iter().map(|field| {
        let field_name = &field.ident.as_ref().unwrap();
        quote! { self.#field_name.kafka_serialize(writer)? }
    });

    let expanded = quote! {
        impl KafkaSerialize for #name {
            fn kafka_byte_len(&self) -> usize {
                0 #(+#byte_lens)*
            }

            fn kafka_serialize<W: std::io::Write>(&self, writer: &mut std::io::BufWriter<W>) -> std::io::Result<()> {
                use std::io::Write;
                #(#buf_writes);*;

                writer.flush()
            }
        }
    };

    TokenStream::from(expanded)
}
