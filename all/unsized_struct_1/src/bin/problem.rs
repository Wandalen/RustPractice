#![feature(type_name_of_val)]

//

fn main()
{

  let prefix = 13;
  // let a = Box::new( UnsizedStruct { prefix, postfix : [ 1, 2, 3 ][ .. ] } );
  // let a : Box< UnsizedStruct > = Box::new( UnsizedStruct { prefix, postfix : [ 1, 2, 3 ] } );
  // let a : Box< UnsizedStruct< [ i32 ] > > = Box::new( UnsizedStruct { prefix, postfix : [ 1, 2, 3 ] } );

  let a = UnsizedStruct::new_from_array( prefix, [ 1, 2 ] );
  wtools::inspect_type_of!( *a );
  assert_eq!( a.prefix, 13 );
  assert_eq!( a.postfix[ 0 ], 1 );
  assert_eq!( a.postfix[ 1 ], 2 );

  let b = UnsizedStruct::new_from_array( prefix, [ 1, 2, 3 ] );
  wtools::inspect_type_of!( *b );
  assert_eq!( b.prefix, 13 );
  assert_eq!( b.postfix[ 0 ], 1 );
  assert_eq!( b.postfix[ 1 ], 2 );
  assert_eq!( b.postfix[ 2 ], 3 );

}

//

#[repr(C)]
#[derive(Debug)]
pub struct UnsizedStruct
{
  prefix : i32,
  postfix : [ i32 ],
}

impl UnsizedStruct
{

  pub fn new_from_array< const N : usize >( prefix : i32, postfix : [ i32; N ] ) -> Box< Self >
  {

    #[repr(C)]
    #[derive(Debug)]
    pub struct UnsizedStruct2< T >
    where
      T : ?Sized,
    {
      prefix : i32,
      postfix : T,
    }

    let r : Box< UnsizedStruct2< [ i32 ] > > = Box::new( UnsizedStruct2 { prefix, postfix } );
    let ptr = Box::into_raw( r ) as *mut Self;

    unsafe
    {
      Box::from_raw( ptr )
    }

  }

  pub fn new_from_slice( prefix : i32, postfix : &[ i32 ] ) -> Box< Self >
  {

    let len = postfix.len();
    assert!( len * 4 <= isize::MAX as usize - 4 );
    let mut v = Vec::with_capacity( len + 1 );
    v.push( prefix );
    v.extend( postfix );
    let data = Box::leak( v.into_boxed_slice() );
    let ptr = &mut data[ ..len ] as *mut [ i32 ] as *mut Self;
    // SAFETY: The address and length metadata were encoded in the `*mut [i32]` pointer.
    unsafe { Box::from_raw(ptr) }

//     #[repr(C)]
//     #[derive(Debug)]
//     pub struct UnsizedStruct2< T >
//     where
//       T : ?Sized,
//     {
//       prefix : i32,
//       postfix : (),
//     }
//
//     let r : Box< UnsizedStruct2< [ i32 ] > > = Box::new( UnsizedStruct2 { prefix, postfix } );
//     let ptr = Box::into_raw( r ) as *mut Self;
//
//     unsafe
//     {
//       Box::from_raw( ptr )
//     }

  }

}

// pub struct UnsizedStruct< T >
// where
//   T : ?Sized,
// {
//   prefix : i32,
//   postfix : T,
// }
