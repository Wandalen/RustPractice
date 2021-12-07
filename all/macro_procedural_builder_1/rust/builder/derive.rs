#![ feature( type_name_of_val ) ]

extern crate proc_macro;
extern crate proc_macro_error;

#[macro_use]
mod wt;
mod builder;

#[proc_macro_derive( Builder, attributes( builder ) )]
pub fn builder( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let result = builder::builder( input );
  // proc_macro::TokenStream::new()
  // result.into()
  match result
  {
    Ok( stream ) => stream.into(),
    Err( err ) => err.to_compile_error().into(),
  }
}

//