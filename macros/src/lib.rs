use heck::ToUpperCamelCase;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse::Parse, punctuated::Punctuated, Ident, LitStr, Token};

struct Input {
    ident: Ident,
    suffix: Option<LitStr>,
    constants: Punctuated<Ident, Token![,]>,
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Input {
            ident: input.parse()?,
            suffix: input.parse()?,
            constants: input.parse_terminated(Ident::parse)?,
        })
    }
}

#[proc_macro]
pub fn gl_enum(tokens: TokenStream) -> TokenStream {
    let Input {
        ident,
        suffix,
        constants,
    } = syn::parse_macro_input!(tokens);

    let mut fields = Vec::new();

    for constant in constants {
        let ident = if let Some(ref suffix) = suffix {
            constant
                .to_string()
                .strip_suffix(&suffix.value())
                .unwrap_or_else(|| {
                    panic!(
                        "Failed to remove suffix {} from constant {constant}",
                        suffix.value()
                    )
                })
                .to_string()
                .to_upper_camel_case()
        } else {
            constant.to_string().to_upper_camel_case()
        };

        let ident = format_ident!("{}", ident.strip_prefix("Gl").unwrap_or(&ident));
        fields.push(quote!(#ident = #constant))
    }

    quote! {
        #[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, IntoPrimitive)]
        #[repr(u32)]
        #[non_exhaustive]
        pub enum #ident {
            #(#fields),*
        }
    }
    .into()
}
