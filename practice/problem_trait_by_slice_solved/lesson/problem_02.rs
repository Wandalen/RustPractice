// https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=68823817984d1c9f94ae1bc713b74974

fn main()
{
  trait Trait1 {}
  impl< T : Sized > Trait1 for &[ T ] {}
  fn does_implement_trait1( _ : &impl Trait1 ) -> bool { true }
  let src = &[ 1, 2, 3 ];
  assert_eq!( does_implement_trait1( &src ), true );
}
