#![ feature( type_name_of_val ) ]

extern crate proc_macro;
extern crate proc_macro_error;

mod wt;
mod builder;

#[proc_macro_derive( Builder )]
pub fn builder( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  builder::builder( input )
}
