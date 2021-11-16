#[test]
fn tests()
{
  let t = trybuild::TestCases::new();
  t.pass( "rust/test/full.rs" );
  // t.pass( "rust/test/01-parse.rs" );
  // t.pass( "rust/test/02-create-builder.rs" );
  // t.pass( "rust/test/03-call-setters.rs" );
  // t.pass(   "rust/test/04-call-build.rs"   );
  // t.pass(   "rust/test/05-method-chaining.rs"   );
  // t.pass(   "rust/test/06-optional-field.rs"   );
  // t.pass(   "rust/test/07-repeated-field.rs"   );
  // t.compile_fail(   "rust/test/08-unrecognized-attribute.rs"   );
  // t.pass(   "rust/test/09-redefined-prelude-types.rs"   );
}
