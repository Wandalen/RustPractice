// #![allow( unused_imports )]

/*
 * ! abs
 * ? def
 */

//
// #![feature( allow_internal_unstable )]
#![feature( core_panic )]
//! #![no_std]

//   - An example of a derive macro implemented using Syn:
//     https://github.com/dtolnay/syn/tree/master/examples/heapsize

#[macro_use]
extern crate maplit;
extern crate former_derive;
extern crate former_runtime;
// extern crate std;
// extern crate alloc;

// mod former
// {
//   pub mod runtime;
//   pub mod test_framework;
// }

// use alloc::vec;
// use std::dbg;
// use std::vec::Vec;
// use std::string::String;

use former_derive::Former;
// use former_runtime::test_framework::*;
use anyhow;

#[derive( Debug, PartialEq )]
#[derive( Former )]
pub struct Command
{
  pub int_1 : i32,
  string_1 : String,
  vec_1 : Vec< String >,
  hashmap_strings_1 : std::collections::HashMap< String, String >,
  int_optional_1 : core::option::Option< i32 >,
  string_optional_1 : Option< String >,
}

//

// impl Command
// {
//   pub fn former() -> CommandFormer
//   {
//     CommandFormer
//     {
//       int_1 : core::option::Option::None,
//       string_1 : core::option::Option::None,
//       vec_1 : core::option::Option::None,
//       hashmap_strings_1 : core::option::Option::None,
//       int_optional_1 : core::option::Option::None,
//       string_optional_1 : core::option::Option::None,
//     }
//   }
// }

// //

// #[derive( Debug )]
// pub struct CommandFormer
// {
//   pub int_1 : core::option::Option< i32 >,
//   pub string_1 : core::option::Option< String >,
//   pub vec_1 : core::option::Option< Vec< String > >,
//   pub hashmap_strings_1 : core::option::Option< std::collections::HashMap< String, String > >,
//   pub int_optional_1 :  core::option::Option< i32 >,
//   pub string_optional_1 : core::option::Option< String >,
// }

//

impl CommandFormer
{
  // fn form( mut self ) -> Command
  // {

  //   let int_1 = if self.int_1.is_some()
  //   {
  //     self.int_1.take().unwrap()
  //   }
  //   else
  //   {
  //     let val : i32 = Default::default();
  //     val
  //   };

  //   let string_1 = if self.string_1.is_some()
  //   {
  //     self.string_1.take().unwrap()
  //   }
  //   else
  //   {
  //     let val : String = Default::default();
  //     val
  //   };

  //   let vec_1 = if self.vec_1.is_some()
  //   {
  //     self.vec_1.take().unwrap()
  //   }
  //   else
  //   {
  //     let val : Vec< String > = Default::default();
  //     val
  //   };

  //   let hashmap_strings_1 = if self.hashmap_strings_1.is_some()
  //   {
  //     self.hashmap_strings_1.take().unwrap()
  //   }
  //   else
  //   {
  //     let val : std::collections::HashMap< String, String > = Default::default();
  //     val
  //   };

  //   let int_optional_1 = if self.int_optional_1.is_some()
  //   {
  //     Some( self.int_optional_1.take().unwrap() )
  //   }
  //   else
  //   {
  //     None
  //   };

  //   let string_optional_1 = if self.string_optional_1.is_some()
  //   {
  //     Some( self.string_optional_1.take().unwrap() )
  //   }
  //   else
  //   {
  //     None
  //   };

  //   Command
  //   {
  //     int_1,
  //     string_1,
  //     vec_1,
  //     hashmap_strings_1,
  //     int_optional_1,
  //     string_optional_1,
  //   }

  // }

  // pub fn int_1< Src >( mut self, src : Src ) -> Self
  // where Src : core::convert::Into< i32 >,
  // {
  //   debug_assert!( self.int_1.is_none() );
  //   self.int_1 = Some( src.into() );
  //   self
  // }

  // pub fn string_1< Src >( mut self, src : Src ) -> Self
  // where Src : core::convert::Into< String >,
  // {
  //   debug_assert!( self.string_1.is_none() );
  //   self.string_1 = Some( src.into() );
  //   self
  // }

  // pub fn vec_1( mut self ) -> former::runtime::VectorFormer
  // <
  //   String,
  //   Vec< String >,
  //   CommandFormer,
  //   impl Fn( &mut CommandFormer, core::option::Option< Vec< String > > )
  // >
  // {
  //   let container = self.vec_1.take();
  //   let on_end = | former : &mut CommandFormer, container : core::option::Option< Vec< String > > |
  //   {
  //     former.vec_1 = container;
  //   };
  //   former::runtime::VectorFormer::new( self, container, on_end )
  // }

  // pub fn hashmap_strings_1( mut self ) -> former::runtime::HashmapFormer
  // <
  //   String,
  //   String,
  //   std::collections::HashMap< String, String >,
  //   CommandFormer,
  //   impl Fn( &mut CommandFormer, core::option::Option< std::collections::HashMap< String, String > > )
  // >
  // {
  //   let container = self.hashmap_strings_1.take();
  //   let on_end = | former : &mut CommandFormer, container : core::option::Option< std::collections::HashMap< String, String > > |
  //   {
  //     former.hashmap_strings_1 = container;
  //   };
  //   former::runtime::HashmapFormer::new( self, container, on_end )
  // }

  // pub fn string_optional_1< Src >( mut self, src : Src ) -> Self
  // where Src : core::convert::Into< String >
  // {
  //   debug_assert!( self.string_optional_1.is_none() );
  //   self.string_optional_1 = Some( src.into() );
  //   self
  // }

}

//

fn test_int() -> anyhow::Result< () >
{

  // test.case( "basic" );

  let command = Command::former()
  .int_1( 13 )
  .form();
  // dbg!( &command );

  let expected = Command
  {
    int_1 : 13,
    string_1 : "".to_string(),
    vec_1 : vec![],
    hashmap_strings_1 : hashmap!{},
    int_optional_1 : None,
    string_optional_1 : None,
  };
  assert_eq!( command, expected );

  // test.case( "rewriting" );

  // should_throw( ||
  // {
  //   let _command = Command::former()
  //   .int_1( 1 )
  //   .int_1( 3 )
  //   .form();
  //   Ok( () )
  // })?;

  Ok( () )
}

//

fn test_string() -> anyhow::Result< () >
{

  // test.case( "string : object" );

  let command = Command::former()
  .string_1( "Abcd".to_string() )
  .form();
  // dbg!( &command );

  let expected = Command
  {
    int_1 : 0,
    string_1 : "Abcd".to_string(),
    vec_1 : vec![],
    hashmap_strings_1 : hashmap!{},
    int_optional_1 : None,
    string_optional_1 : None,
  };
  assert_eq!( command, expected );

  // test.case( "string : slice" );

  let command = Command::former()
  .string_1( "Abcd" )
  .form();
  // dbg!( &command );

  let expected = Command
  {
    int_1 : 0,
    string_1 : "Abcd".to_string(),
    vec_1 : vec![],
    hashmap_strings_1 : hashmap!{},
    int_optional_1 : None,
    string_optional_1 : None,
  };
  assert_eq!( command, expected );

  // test.case( "string : rewriting" );

  // should_throw( ||
  // {
  //   let _command = Command::former()
  //   .string_1( "dir1" )
  //   .string_1( "dir2" )
  //   .form();
  //   Ok( () )
  // })?;

  Ok( () )
}

//

fn test_vector() -> anyhow::Result< () >
{

  // test.case( "vector : implicit construction" );

  let command = Command::former()
  .vec_1().push( "ghi" ).push( "klm" ).end()
  .form()
  ;
  // dbg!( &command );

  let expected = Command
  {
    int_1 : 0,
    string_1 : "".to_string(),
    vec_1 : vec![ "ghi".to_string(), "klm".to_string() ],
    hashmap_strings_1 : hashmap!{},
    int_optional_1 : None,
    string_optional_1 : None,
  };
  assert_eq!( command, expected );

  // test.case( "vector : replace" );

  let command = Command::former()
  .vec_1().replace( vec![ "a".to_string(), "bc".to_string(), "def".to_string() ] ).end()
  .form();
  // dbg!( &command );

  let expected = Command
  {
    int_1 : 0,
    string_1 : "".to_string(),
    vec_1 : vec![ "a".to_string(), "bc".to_string(), "def".to_string() ],
    hashmap_strings_1 : hashmap!{},
    int_optional_1 : None,
    string_optional_1 : None,
  };
  assert_eq!( command, expected );

  // test.case( "vector : replace and push" );

  let command = Command::former()
  .vec_1().replace( vec![ "a".to_string(), "bc".to_string(), "def".to_string() ] ).push( "gh" ).end()
  .form();
  // dbg!( &command );

  let expected = Command
  {
    int_1 : 0,
    string_1 : "".to_string(),
    vec_1 : vec![ "a".to_string(), "bc".to_string(), "def".to_string(), "gh".to_string() ],
    hashmap_strings_1 : hashmap!{},
    int_optional_1 : None,
    string_optional_1 : None,
  };
  assert_eq!( command, expected );

  Ok( () )
}

//

fn test_hashmap() -> anyhow::Result< () >
{

  // test.case( "implicit construction" );

  let command = Command::former()
  .hashmap_strings_1().insert( "k1", "v1" ).insert( "k2", "v2" ).end()
  .form()
  ;
  // dbg!( &command );

  let expected = Command
  {
    int_1 : 0,
    string_1 : "".to_string(),
    vec_1 : vec![],
    hashmap_strings_1 : hashmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() },
    int_optional_1 : None,
    string_optional_1 : None,
  };
  assert_eq!( command, expected );

  // test.case( "replace" );

  let command = Command::former()
  .hashmap_strings_1().replace( hashmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() } ).end()
  .form()
  ;
  // dbg!( &command );

  let expected = Command
  {
    int_1 : 0,
    string_1 : "".to_string(),
    vec_1 : vec![],
    hashmap_strings_1 : hashmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() },
    int_optional_1 : None,
    string_optional_1 : None,
  };
  assert_eq!( command, expected );

  // test.case( "replace and insert" );

  let command = Command::former()
  .hashmap_strings_1().replace( hashmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string() } ).insert( "k3", "v3" ).end()
  .form()
  ;
  // dbg!( &command );

  let expected = Command
  {
    int_1 : 0,
    string_1 : "".to_string(),
    vec_1 : vec![],
    hashmap_strings_1 : hashmap!{ "k1".to_string() => "v1".to_string(), "k2".to_string() => "v2".to_string(), "k3".to_string() => "v3".to_string() },
    int_optional_1 : None,
    string_optional_1 : None,
  };
  assert_eq!( command, expected );

  Ok( () )
}

//

fn test_optional_string() -> anyhow::Result< () >
{

  // test.case( "basic" );

  let command = Command::former()
  .string_optional_1( "dir1" )
  .form();
  dbg!( &command );

  let expected = Command
  {
    int_1 : 0,
    string_1 : "".to_string(),
    vec_1 : vec![],
    hashmap_strings_1 : hashmap!{},
    int_optional_1 : None,
    string_optional_1 : Some( "dir1".to_string() ),
  };
  assert_eq!( command, expected );

  // test.case( "none" );

  let command = Command::former()
  .form();
  dbg!( &command );

  let expected = Command
  {
    int_1 : 0,
    string_1 : "".to_string(),
    vec_1 : vec![],
    hashmap_strings_1 : hashmap!{},
    int_optional_1 : None,
    string_optional_1 : None,
  };
  assert_eq!( command, expected );

  // test.case( "optional : rewriting" );

  // should_throw( ||
  // {
  //   let _command = Command::former()
  //   .string_optional_1( "dir1" )
  //   .string_optional_1( "dir2" )
  //   .form();
  //   Ok( () )
  // })?;

  Ok( () )
}

//

fn main()
{
  test_int().unwrap();
  test_string().unwrap();
  test_vector().unwrap();
  test_hashmap().unwrap();
  test_optional_string().unwrap();
}

//

#[ test ]
fn main_test()
{
  test_int().unwrap();
  test_string().unwrap();
  test_vector().unwrap();
  test_hashmap().unwrap();
  test_optional_string().unwrap();
}
