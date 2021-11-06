// https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=b993179e3d69a19a619e20bd3ad76c16

fn main()
{
  trait Trait1 {}
  fn does_implement_trait1( _ : &impl Trait1 ) -> bool { true }
  impl< T : Sized > Trait1 for &[ T ] {}
  // impl< T : Sized, const N : usize > Trait1 for &[ T; N ] {}
  let src1 : &[ i32 ] = &[ 1, 2, 3 ];
  assert_eq!( does_implement_trait1( &src1 ), true );
  let src2 = &[ 1, 2, 3 ];
  assert_eq!( does_implement_trait1( &&src2[ .. ] ), true );
}
