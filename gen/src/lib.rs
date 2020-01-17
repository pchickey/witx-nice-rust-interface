extern crate proc_macro;

mod parse;
mod types;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro]
pub fn gen(args: TokenStream) -> TokenStream {
    let args = TokenStream2::from(args);
    let witx_paths = parse::witx_paths(args).expect("parsing macro arguments");
    let doc = witx::load(&witx_paths).expect("loading witx");
    let out = types::gen(doc).unwrap();
    TokenStream::from(out)
}
