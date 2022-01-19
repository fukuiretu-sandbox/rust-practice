use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = dbg!(item);
    item
}
