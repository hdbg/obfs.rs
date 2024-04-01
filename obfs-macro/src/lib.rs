use proc_macro::{self, TokenStream};

#[proc_macro]
pub fn confuse_flow(stream: TokenStream) -> TokenStream {
    let x = stream.clone();
    let parse = syn::parse_macro_input!(stream as syn::Block);
    println!("{:#?}", parse);
    x
}
