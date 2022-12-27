use proc_macro::TokenStream;

mod list;
mod tree;

#[proc_macro]
pub fn binary_tree(input: TokenStream) -> TokenStream {
    proc_macro::TokenStream::from(tree::from(proc_macro2::TokenStream::from(input)))
}

#[proc_macro]
pub fn single_list(input: TokenStream) -> TokenStream {
    proc_macro::TokenStream::from(list::from(proc_macro2::TokenStream::from(input)))
}
