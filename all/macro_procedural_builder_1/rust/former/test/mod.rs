#[test]
fn tests()
{
  let t = trybuild::TestCases::new();
  t.pass( "rust/test/full.rs" );
}
