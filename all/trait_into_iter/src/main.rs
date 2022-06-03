#![feature(type_name_of_val)]

use wtools::typing::inspect_type_of;

//
// NOTE : Important to understand that trait IntoIter might also be used without consumption of the original content.
//

fn handle< I, Item >( _iter : I )
where
  I : IntoIterator< Item = Item >,
  Item : core::fmt::Debug,
{
  for item in _iter
  {
    inspect_type_of!( item );
    // dbg!( item );
  }
}

//

fn main()
{

  #[ derive( Debug ) ]
  struct NoCopy< T >( T );

  let vec = vec!( NoCopy( 1 ), NoCopy( 2 ), NoCopy( 3 ) );
  handle( &vec );
  // dbg!( &vec );
  // handle( vec );

  for item in &vec
  {
    inspect_type_of!( item );
  }

  for item in vec
  {
    inspect_type_of!( item );
  }

}
