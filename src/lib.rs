extern crate proc_macro;
extern crate syn;
extern crate time;

use proc_macro::TokenStream;

#[proc_macro]
pub fn build_time(input: TokenStream) -> TokenStream {
    // TODO proper error handling

    // TODO support passing the name of the generated const as an argument

    let fmtstr: syn::LitStr = syn::parse(input).unwrap();

    let time = time::now_utc();

    let ftime = time::strftime(&fmtstr.value(), &time).unwrap();

    let mut out_str = String::new();

    out_str.push_str("const BUILD_TIME: &str = \"");
    out_str.push_str(&ftime);
    out_str.push_str("\";");

    out_str.parse().unwrap()
}
