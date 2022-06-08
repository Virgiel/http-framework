use proc_macro::TokenStream;

#[proc_macro]
pub fn route(input: TokenStream) -> TokenStream {
    println!("Hello, world!");
    dbg!(&input);
    input
}
