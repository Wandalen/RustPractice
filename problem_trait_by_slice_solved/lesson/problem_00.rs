/* https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=6db54bc4f73ffcffe7b34530208e0f0c */

fn main()
{
  trait Trait1 {}
  fn does_implement_trait1( _ : &impl Trait1 ) -> bool { true }
  impl< T : Sized > Trait1 for &[ T ] {}
  let src = &[ 1, 2, 3 ];
  assert_eq!( does_implement_trait1( &src ), true ); /* does not work */
//   /* but that works : */
//   let src : &[ i32 ] = &[ 1, 2, 3 ];
//   assert_eq!( does_implement_trait1( &src ), true );
}
