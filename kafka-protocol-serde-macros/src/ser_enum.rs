use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::DataEnum;

pub fn impl_serialize_for_res_enum(name: &Ident, enum_data: &DataEnum) -> TokenStream {
    let variants_byte_lens = enum_data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        quote! {
            Self::#variant_name { header, body } => header.kafka_byte_len() + body.kafka_byte_len()
        }
    });

    let variants_serializers = enum_data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        quote! {
            Self::#variant_name { header, body } => {
                header.kafka_serialize(writer)?;
                body.kafka_serialize(writer)?;
                writer.flush()
            }
        }
    });

    let expanded = quote! {
        impl KafkaSerialize for #name {
            fn kafka_byte_len(&self) -> usize {
                match self {
                    #(#variants_byte_lens),*
                }
            }

            fn kafka_serialize<W: std::io::Write>(&self, writer: &mut std::io::BufWriter<W>) -> std::io::Result<()> {
                match self {
                    #(#variants_serializers),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
