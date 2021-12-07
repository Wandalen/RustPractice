#![allow(unused_imports)]
#![allow(dead_code)]

use syn::{ parse_macro_input, parse_quote, DeriveInput };
use syn::spanned::Spanned;
use quote::quote;
use proc_macro_error::{ abort, abort_call_site };
use unzip_n::unzip_n;
// use crate::wt;
use crate::wt;

unzip_n!( 3 );
unzip_n!( 4 );
unzip_n!( 5 );
unzip_n!( 6 );
unzip_n!( 7 );

//

#[derive( Debug )]
enum ContainerKind
{
  No,
  Vector,
  HashMap,
}

//

#[allow( dead_code )]
struct FormerField< 'a >
{
  pub attrs : &'a Vec< syn::Attribute >,
  pub vis : &'a syn::Visibility,
  pub name_ident : &'a Option< syn::Ident >,
  pub colon_token : &'a Option< syn::token::Colon >,
  pub ty : &'a syn::Type,
  pub non_optional_ty : &'a syn::Type,
  pub is_option : bool,
  pub is_container : ContainerKind,
}

#[allow(unused_macros)]
macro_rules! tree_print
{
  ( $src : expr ) =>
  {{
    println!( "{}", tree_export_str!( $src ) );
  }};
  ( $( $src : expr ),+ $(,)? ) =>
  {{
    $( tree_print!( $src ) );+;
  }};
}

#[allow(unused_macros)]
macro_rules! tree_export_str
{
  ( $src : expr ) =>
  {{
    let src2 = &$src;
    format!( "{} : {} :\n{:#?}", stringify!( $src ), quote!{ #src2 }, $src )
  }};
}

//

fn is_option( ty : &syn::Type ) -> bool
{
  if let syn::Type::Path( path ) = ty
  {
    let last = &path.path.segments.last();
    if last.is_none()
    {
      return false;
    }
    return last.unwrap().ident == "Option";
  }
  false
}

//

fn parameters_internal( ty : &syn::Type, r : core::ops::RangeInclusive< usize > ) -> Vec< &syn::Type >
{
  // return vec![];
  if let syn::Type::Path( syn::TypePath{ path : syn::Path { ref segments, .. }, .. } ) = ty
  {
    let last = &segments.last();
    if last.is_none()
    {
      return vec![ &ty ]
    }
    let args = &last.unwrap().arguments;
    if let syn::PathArguments::AngleBracketed( ref args2 ) = args
    {
      let args3 = &args2.args;
      // trace_macros!( true );
      let selected : Vec< &syn::Type > = args3
      .iter()
      .skip_while( | e | if let syn::GenericArgument::Type( _ ) = e { false } else { true } )
      .skip( *r.start() )
      .take( *r.end() - *r.start() + 1 )
      .map( | e | if let syn::GenericArgument::Type( ty ) = e { ty } else { unreachable!( "Expects Type" ) } )
      .collect();
      // tree_print!( selected.first().unwrap() );
      return selected;
    }
  }
  vec![ &ty ]
}

//

fn parameter_internal_first( ty : &syn::Type ) -> &syn::Type
{
  parameters_internal( ty, 0 ..= 0 )
  .first()
  .or_else( || panic!( "Expect at least one parameter here:\n  {}", quote!{ #ty } ) ).unwrap()
}

//

fn parameter_internal_first_two( ty : &syn::Type ) -> ( &syn::Type, &syn::Type )
{
  let result = parameters_internal( ty, 0 ..= 1 );
  let mut iter = result.iter();
  (
    iter.next().or_else( || panic!( "Expect at least two parameters here:\n  {}", quote!{ #ty } ) ).unwrap(),
    iter.next().or_else( || panic!( "Expect at least two parameters here:\n  {}", quote!{ #ty } ) ).unwrap(),
  )
}

//

fn is_container( ty : &syn::Type ) -> ContainerKind
{
  if let syn::Type::Path( path ) = ty
  {
    let last = &path.path.segments.last();
    if last.is_none()
    {
      return ContainerKind::No
    }
    match last.unwrap().ident.to_string().as_ref()
    {
      "Vec" => { return ContainerKind::Vector }
      "HashMap" => { return ContainerKind::HashMap }
      _ => { return ContainerKind::No }
    }
  }
  ContainerKind::No
}

//

#[inline]
fn field_none_map( field : &FormerField ) -> syn::Field
{
  // let attrs = field.attrs.clone();
  // let _vis = field.vis.clone();
  let name_ident = field.name_ident.clone();
  let colon_token = field.colon_token.clone();
  // let _ty = field.ty.clone();

  let tokens = quote! { core::option::Option::None };
  let ty2 : syn::Type = syn::parse2( tokens ).unwrap();
  syn::Field
  {
    attrs : vec![],
    vis : syn::parse2( quote!{} ).unwrap(),
    ident : name_ident,
    colon_token,
    ty : ty2,
  }
}

//

#[inline]
fn field_optional_map( field : &FormerField ) -> syn::Field
{
  // let attrs = field.attrs.clone();
  // let _vis = field.vis.clone();
  let name_ident = field.name_ident.clone();
  let colon_token = field.colon_token.clone();
  let ty = field.ty.clone();

  let tokens = if is_option( &ty )
  {
    quote! { #ty }
  }
  else
  {
    quote! { core::option::Option< #ty > }
  };

  let ty2 : syn::Type = syn::parse2( tokens ).unwrap();

  syn::Field
  {
    // attrs,
    attrs : vec![],
    vis : syn::parse2( quote!{} ).unwrap(),
    ident : name_ident,
    colon_token,
    ty : ty2,
  }
}

//

#[inline]
fn field_build_map( field : &FormerField ) -> syn::Stmt
{
  let name_ident = field.name_ident;
  let ty = field.ty;

  let tokens = if field.is_option
  {
    quote!
    {
      let #name_ident = if self.#name_ident.is_some()
      {
        core::option::Option::Some( self.#name_ident.take().unwrap() )
      }
      else
      {
        core::option::Option::None
      };
    }
  }
  else
  {
    quote!
    {
      let #name_ident = if self.#name_ident.is_some()
      {
        self.#name_ident.take().unwrap()
      }
      else
      {
        let val : #ty = Default::default();
        val
      };
    }
  };

  let stmt : syn::Stmt = syn::parse2( tokens ).unwrap();
  stmt
}

//

#[inline]
fn field_name_map( field : &FormerField ) -> syn::Ident
{
  let name_ident = field.name_ident.clone();
  if let Some( name_ident ) = name_ident
  {
    name_ident
  }
  else
  {
    syn::Ident::new( "?? no name ??", proc_macro2::Span::call_site() )
  }
}

//

#[inline]
fn field_setter_map( field : &FormerField ) -> syn::Stmt
{
  let name_ident = field.name_ident.clone();
  // let ty = field.ty;

  let tokens =
  {
    let non_optional_ty = &field.non_optional_ty;
    quote!
    {
      pub fn #name_ident< Src >( &mut self, src : Src ) -> &mut Self
      where Src : core::convert::Into< #non_optional_ty >,
      {
        debug_assert!( self.#name_ident.is_none() );
        self.#name_ident = core::option::Option::Some( src.into() );
        self
      }
    }
  };

  let stmt : syn::Stmt = syn::parse2( tokens ).unwrap();
  stmt

  // pub fn int_1< Src >( mut self, src : Src ) -> Self
  // where Src : core::convert::Into< i32 >,
  // {
  //   debug_assert!( self.int_1.is_none() );
  //   self.int_1 = Some( src.into() );
  //   self
  // }

}

//

struct AttrEach
{
  // bracket_token : syn::token::Bracket,
  bracket_token : syn::token::Paren,
  each_ident : syn::Ident,
  assign_token : syn::Token![ = ],
  name : syn::LitStr,
}

impl syn::parse::Parse for AttrEach
{
  fn parse( input : syn::parse::ParseStream ) -> Result< Self, syn::Error >
  {
    let content;
    Ok( AttrEach
    {
      bracket_token : syn::parenthesized!( content in input ),
      each_ident : content.parse()?,
      assign_token : content.parse()?,
      name : content.parse()?,
    })
  }
}

//

#[inline]
fn field_setter_attr_map( field : &FormerField ) -> Result< Option< syn::Stmt >, syn::Error >
{
  let name_ident = field.name_ident.clone().unwrap();

  for attr in field.attrs
  {
    // println!( "{:?}", field );
    // tree_print!( attr.tokens );

    let meta = attr.parse_meta()?;
    // tree_print!( meta );

    let ( lit_str, path );
    match meta
    {
      syn::Meta::List( ref meta_list ) => match meta_list.nested.first()
      {
        Some( nested_meta ) => match nested_meta
        {
          syn::NestedMeta::Meta( meta2 ) => match meta2
          {
            syn::Meta::NameValue( name_value ) => match &name_value.lit
            {
              syn::Lit::Str( _lit_str ) =>
              {
                path = name_value.path.get_ident().unwrap().to_string();
                lit_str = _lit_str.clone();
              }
              _ => return Err( syn::Error::new( attr.span(), "Unknown format of attribute, expected syn::Lit::Str( lit_str )" ) ),
            },
            _ => return Err( syn::Error::new( attr.span(), "Unknown format of attribute, expected syn::Meta::NameValue( name_value )" ) ),
          },
          _ => return Err( syn::Error::new( attr.span(), "Unknown format of attribute, expected syn::NestedMeta::Meta( meta2 )" ) ),
        },
        _ => return Err( syn::Error::new( attr.span(), "Unknown format of attribute, expected Some( nested_meta )" ) ),
      },
      _ => return Err( syn::Error::new( attr.span(), "Unknown format of attribute, expected syn::Meta::List( meta_list )" ) ),
    };

    if path != "each"
    {
      let error = syn::Error::new( attr.span(), "expected `builder( each = \"...\" )`" );
      return Err( error );
    }

    let name2_ident = syn::Ident::new( lit_str.value().as_ref(), meta.span() );

    if name2_ident.to_string() == name_ident.to_string()
    {
      return Ok( None );
    }

    let tokens =
    {
      let internal_ty = parameter_internal_first( field.ty );
      quote!
      {
        pub fn #name2_ident< Src >( &mut self, src : Src ) -> &mut Self
        where Src : core::convert::Into< #internal_ty >,
        {
          if self.#name_ident.is_none()
          {
            self.#name_ident = core::option::Option::Some( Default::default() );
          }
          if let core::option::Option::Some( some ) = &mut self.#name_ident
          {
            some.push( src.into() );
          }
          self
        }
      }
    };

    let stmt : syn::Stmt = syn::parse2( tokens ).unwrap();
    return Ok( Some( stmt ) );

//     let attr_each : AttrEach = syn::parse2( attr.tokens.clone() ).expect( "Could not parse attr" );
//
//     if attr_each.each_ident.to_string() != "each"
//     {
//       let error = syn::Error::new( attr.span(), "expected `builder( each = \"...\" )`" );
//       return Err( error );
//     }
//
//     // if let proc_macro2::TokenTree::Group( group ) = attr.tokens.clone().into_iter().next().unwrap()
//     {
//       // tree_print!( group );
//       // let attr_each : AttrEach = syn::parse2( group.stream() ).expect( "Could not parse attr" );
//
//       // println!( "each_ident : {}", attr_each.each_ident.to_string() );
//       // println!( "name : {}", attr_each.name.value() );
//
//       let name2_ident = syn::Ident::new( attr_each.name.value().as_ref(), attr_each.name.span() );
//
//       if name2_ident.to_string() == name_ident.to_string()
//       {
//         return Ok( None );
//       }
//
//       let tokens =
//       {
//         // let ty = &field.ty;
//         let internal_ty = parameter_internal_first( field.ty );
//         // let non_optional_ty = &field.non_optional_ty;
//         quote!
//         {
//           pub fn #name2_ident< Src >( &mut self, src : Src ) -> &mut Self
//           where Src : core::convert::Into< #internal_ty >,
//           {
//             if self.#name_ident.is_none()
//             {
//               self.#name_ident = core::option::Option::Some( Default::default() );
//             }
//             if let core::option::Option::Some( some ) = &mut self.#name_ident
//             {
//               some.push( src.into() );
//             }
//             self
//           }
//         }
//       };
//
//       let stmt : syn::Stmt = syn::parse2( tokens ).unwrap();
//       return Ok( Some( stmt ) );
//     }

  }

  return Ok( None );
}

//

pub fn builder( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream, syn::Error >
{
  // let ast = parse_macro_input!( input as DeriveInput );

  let ast = match syn::parse::< syn::DeriveInput >( input )
  {
    Ok( syntax_tree ) => syntax_tree,
    Err( err ) => return Err( err ),
  };
  let name = &ast.ident;

  let builder_name = format!( "{}Former", name );
  let builder_ident = syn::Ident::new( &builder_name, name.span() );
  let fields = if let syn::Data::Struct( syn::DataStruct { fields : syn::Fields::Named( syn::FieldsNamed { ref named, .. } ), .. } ) = ast.data
  {
    named
  }
  else
  {
    abort!( ast.ident.span(), "Expects struct" );
  };

  let ( fields_none, fields_optional, fields_build, fields_names, fields_setter, fields_setter2 ) = fields.iter().map( | field |
  {
    let attrs = &field.attrs;
    let vis = &field.vis;
    let name_ident = &field.ident;
    let colon_token = &field.colon_token;
    let ty = &field.ty;
    let is_option = is_option( &ty );
    let is_container = is_container( &ty );
    let non_optional_ty : &syn::Type = if is_option { parameter_internal_first( ty ) } else { ty };

    let builder_field = FormerField { attrs, vis, name_ident, colon_token, ty, non_optional_ty, is_option, is_container };
    (
      field_none_map( &builder_field ),
      field_optional_map( &builder_field ),
      field_build_map( &builder_field ),
      field_name_map( &builder_field ),
      field_setter_map( &builder_field ),
      field_setter_attr_map( &builder_field ),
    )
  }).unzip_n_vec();

  let error1 = fields_setter2.iter().find( | e | e.is_err() );

  // if let Some( Err( error2 ) ) = &&error1
  // if error1.is_some()
  // {
  //   return Err( error1.unwrap().unwrap_err().clone() );
  //   // return Err( error2.clone() );
  // }

  if let Some( Err( error2 ) ) = &&error1
  {
    // println!( "!!!!!!!!!!!!!!!!!" );
    return Err( error2.clone() );
  }

  let fields_setter2 = fields_setter2.into_iter().filter_map( | e | e.unwrap() ).collect::< Vec< syn::Stmt > >();

  let result = quote!
  {

    impl #name
    {
      pub fn builder() -> #builder_ident
      {
        #builder_ident
        {
          #( #fields_none, )*
        }
      }
    }

    #[derive( Debug )]
    pub struct #builder_ident
    {
      #( #fields_optional, )*
    }

    impl #builder_ident
    {
      pub fn build( &mut self ) -> core::result::Result< #name, std::boxed::Box< dyn std::error::Error > >
      {
        #( #fields_build )*
        let result = #name
        {
          #( #fields_names, )*
        };
        Ok( result )
      }

      #( #fields_setter )*
      #( #fields_setter2 )*

    }

  };

  // println!( "{:#?}", ast );
  // println!( "{:#?}", result );
  // tree_print!( selected );

  // proc_macro::TokenStream::new()
  // result.into()
  Ok( result )
}
