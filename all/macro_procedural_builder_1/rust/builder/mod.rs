extern crate proc_macro;

mod builder;

#[proc_macro_derive( Builder )]
pub fn builder( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  builder::builder( input )
}
