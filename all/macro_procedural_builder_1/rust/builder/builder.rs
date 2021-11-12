use quote::quote;
use syn::{ parse_macro_input, DeriveInput };

pub fn builder( input : proc_macro::TokenStream ) -> proc_macro::TokenStream
{
  let ast = parse_macro_input!( input as DeriveInput );
  let name = &ast.ident;

  let bname = format!( "{}Builder", name );
  let bident = syn::Ident::new( &bname, name.span() );
  let fields = if let syn::Data::Struct( syn::DataStruct { fields : syn::Fields::Named( syn::FieldsNamed { ref named, .. } ), .. } ) = ast.data
  {
    dbg!( &named );
  }
  else
  {
    abort!( ast.data, "Expects struct" );
  }
  let result = quote!
  {
    struct #bident
    {
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
