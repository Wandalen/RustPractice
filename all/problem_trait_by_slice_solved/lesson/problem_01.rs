// https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=807aabe00f97c3b184da78379720d5b8

fn main()
{
  trait Trait1 {}
  impl< T : Sized > Trait1 for &[ T ] {}
  fn does_implement_trait1( _ : &impl Trait1 ) -> bool { true }
  let src : &[ i32 ] = &[ 1, 2, 3 ];
  assert_eq!( does_implement_trait1( &src ), true );
}
