use proc_macro::TokenStream;
use syn::DeriveInput;

fn impl_reflective_trait(ast: DeriveInput) -> TokenStream {
    // get struct identifier
    let ident = ast.ident;
    let ident_str = ident.to_string();

    // generate impl
    quote::quote! {
        impl Reflective for #ident {
            fn name(&self) -> &'static str {
                #ident_str
            }
        }
    }
    .into()
}

#[proc_macro_derive(Reflective)]
pub fn reflective_derive_macro(item: TokenStream) -> TokenStream {
    // parse
    let ast: DeriveInput = syn::parse(item).unwrap();

    // generate
    impl_reflective_trait(ast)
}
