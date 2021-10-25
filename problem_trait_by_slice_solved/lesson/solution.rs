// https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=5d346aca864d987e122468807c38800a

fn main()
{
  trait Trait1 {}
  fn does_implement_trait1( _ : &impl Trait1 ) -> bool { true }
  impl< T : Sized > Trait1 for &[ T ] {}
  impl< T : Sized, const N : usize > Trait1 for &[ T; N ] {}
  let src1 : &[ i32 ] = &[ 1, 2, 3 ];
  assert_eq!( does_implement_trait1( &src1 ), true );
  let src2 = &[ 1, 2, 3 ];
  assert_eq!( does_implement_trait1( &src2 ), true );
}
