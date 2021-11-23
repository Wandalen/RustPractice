// use crate::wt;
use quote::quote;
use syn::{ parse_macro_input, DeriveInput };
use proc_macro_error::{ abort };

// // #[ macro_export ]
// macro_rules! inspect_type_of
// {
//   ( $src : expr ) =>
//   {{
//     let mut result = String::new();
//     let stringified = stringify!( $src );

//     result.push_str( &format!( "= {} at {}:{}", stringified, file!(), line!() ) );

//     let size = &std::mem::size_of_val( &$src ).to_string()[ .. ];
//     let name = std::any::type_name_of_val( &$src );
//     result.push_str( &format!( "\n  sizeof( {} ) = {}",name, size )[ .. ] );

//     let size = &std::mem::size_of_val( &&$src ).to_string()[ .. ];
//     let name = std::any::type_name_of_val( &&$src );
//     result.push_str( &format!( "\n  sizeof( {} ) = {}",name, size )[ .. ] );

//     result
//   }};
//   ( $( $src : expr ),+ $(,)? ) =>
//   {
//     ( $( $crate::dbg!( $src ) ),+ )
//   };
// }

// //

// // #[ macro_export ]
// macro_rules! inspect_logging_type_of
// {
//   ( $src : expr ) =>
//   {{
//     let result = inspect_type_of!( $src );
//     println!( "{}", result );
//   }}
// }

//

pub fn former( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let ast = parse_macro_input!( input as DeriveInput );
  let name = &ast.ident;

  let bname = format!( "{}Builder", name );
  let bident = syn::Ident::new( &bname, name.span() );
  let fields = if let syn::Data::Struct( syn::DataStruct { fields : syn::Fields::Named( syn::FieldsNamed { ref named, .. } ), .. } ) = ast.data
  {
    // dbg!( &named );
    // inspect_logging_type_of!( named );
    named
  }
  else
  {
    // unimplemented!()
    // proc_macro::TokenStream::new()
    abort!( ast.ident.span(), "Expects struct" );
    // unimplemented!()
  };

  let fields_optional = fields.iter().map( | field |
  {
    let attrs = field.attrs.clone();
    let vis = field.vis.clone();
    let ident = field.ident.clone();
    let colon_token = field.colon_token;
    let ty = field.ty.clone();
    let ty2 = quote! { std::option::Option< #ty > };
    let ty3 = syn::parse2( ty2 ).unwrap();
    syn::Field
    {
      attrs,
      vis,
      ident,
      colon_token,
      ty : ty3,
    }
  });

  // let fields_init = fields.iter().map( | field |
  // {
  //   let attrs = field.attrs.clone();
  //   let vis = field.vis.clone();
  //   let ident = field.ident.clone();
  //   let colon_token = field.colon_token;
  //   let ty = field.ty.clone();
  //   let ty2 = quote! { std::option::Option< #ty > };
  //   let ty3 = syn::parse2( ty2 ).unwrap();
  //   syn::Field
  //   {
  //     attrs,
  //     vis,
  //     ident,
  //     colon_token,
  //     ty : ty3,
  //   }
  // });

  // executable : core::option::Option::None,

  let result = quote!
  {
    struct #bident
    {
      // #fields
      #(#fields_optional,)*
    }
    impl #name
    {
      pub fn builder() -> #bident
      {
        #bident {  }
      }
    }
  };

  // println!( "{:#?}", ast );
  // println!( "{:#?}", result );

  // proc_macro::TokenStream::new()
  result.into()
}
