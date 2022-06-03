#![ feature( type_name_of_val ) ]

use core::fmt::Debug;

//

fn main()
{

  let prefix = -10_i8;
  let got = UnsizedStruct::make_from_array( prefix, [ -1, -2 ] );
  wtools::inspect_type_of!( *got );
  assert_eq!( got.prefix, -10 );
  assert_eq!( got.postfix[ 0 ], -1 );
  assert_eq!( got.postfix[ 1 ], -2 );
  dbg!( &got );

  let prefix = -10_i8;
  let got = UnsizedStruct::make_from_array( prefix, [ -1, -2, -3 ] );
  wtools::inspect_type_of!( *got );
  assert_eq!( got.prefix, -10 );
  assert_eq!( got.postfix[ 0 ], -1 );
  assert_eq!( got.postfix[ 1 ], -2 );
  assert_eq!( got.postfix[ 2 ], -3 );
  dbg!( &got );

  let prefix = -10_i8;
  let got = UnsizedStruct::make_from_slice( prefix, &[ -1, -2, -3 ][ .. ] );
  wtools::inspect_type_of!( *got );
  assert_eq!( got.prefix, -10 );
  assert_eq!( got.postfix[ 0 ], -1 );
  assert_eq!( got.postfix[ 1 ], -2 );
  assert_eq!( got.postfix[ 2 ], -3 );
  dbg!( &got );

}

//

#[ repr( C ) ]
#[ derive( Debug ) ]
pub struct UnsizedStruct< Prefix, Postfix >
where
  Prefix : Sized,
{
  prefix : Prefix,
  postfix : [ Postfix ],
}

//

impl< Prefix, Postfix > UnsizedStruct< Prefix, Postfix >
where
  Postfix : Copy,
  Prefix : Sized,
{

  #[ inline ]
  pub fn make_from_array< const N : usize >( prefix : Prefix, postfix : [ Postfix; N ] ) -> Box< Self >
  {

    #[ repr( C ) ]
    #[ derive( Debug ) ]
    pub struct SizedStruct< Prefix, T >
    where
      Prefix : Sized,
      T : ?Sized,
    {
      prefix : Prefix,
      postfix : T,
    }

    let r : Box< SizedStruct< Prefix, [ Postfix ] > > = Box::new( SizedStruct { prefix, postfix } );
    let ptr = Box::into_raw( r ) as *mut Self;

    // Safety : just boxing a pointer.
    unsafe
    {
      Box::from_raw( ptr )
    }

  }

  //

  #[ inline ]
  pub fn make_from_slice( prefix : Prefix, postfix : &[ Postfix ] ) -> Box< Self >
  {
    use std::alloc::alloc;
    use core::alloc::Layout;
    use core::ptr::slice_from_raw_parts;

    let prefix_layout = Layout::new::<Prefix>();
    let postfix_layout = Layout::array::< Postfix >( postfix.len() ).expect( "No layout for postfix" );
    let ( layout, _postfix_offset ) = prefix_layout.extend( postfix_layout ).expect( "No layout for the structure" );
    let layout = layout.pad_to_align();

    // Safety : just boxing a pointer.
    let mut result : Box< Self > = unsafe
    {
      let ptr : *mut u8 = alloc( layout );
      let slice = slice_from_raw_parts( ptr, postfix.len() );
      core::mem::transmute::< _, Box< Self > >( slice )
    };

    result.prefix = prefix;
    result.postfix.copy_from_slice( postfix );
    result
  }

}
