use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(EnumVariants)]
pub fn enum_variants(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    
    let match_arms = if let Data::Enum(data_enum) = input.data {
        data_enum.variants.iter().map(|v| {
            let variant_name = &v.ident;
            let variant_str = variant_name.to_string();
            match &v.fields {
                Fields::Unit => quote! { Self::#variant_name => #variant_str },
                Fields::Unnamed(_) => quote! { Self::#variant_name(..) => #variant_str },
                Fields::Named(_) => quote! { Self::#variant_name { .. } => #variant_str },
            }
        }).collect::<Vec<_>>()
    } else {
        panic!("EnumVariantName can only be derived for enums");
    };
    
    let gen = quote! {
        impl #name {
            pub fn variant_name(&self) -> &'static str {
                match self {
                    #(#match_arms),*
                }
            }
        }
    };
    
    gen.into()
}