use std::env;

#[test]
fn tests_trybuild()
{
  println!( "current_dir : {:?}", env::current_dir().unwrap() );
  let t = trybuild::TestCases::new();
  t.pass( "../../../rust/builder/test/01-parse.rs" );
  t.pass( "../../../rust/builder/test/02-create-builder.rs" );
  t.pass( "../../../rust/builder/test/03-call-setters.rs" );
  t.pass( "../../../rust/builder/test/04-call-build.rs" );
  t.pass( "../../../rust/builder/test/05-method-chaining.rs" );
  t.pass( "../../../rust/builder/test/06-optional-field.rs" );
  t.pass( "../../../rust/builder/test/07-repeated-field.rs" );
  t.compile_fail( "../../../rust/builder/test/08-unrecognized-attribute.rs" );
  t.pass( "../../../rust/builder/test/09-redefined-prelude-types.rs" );
}

// include!( "../../../rust/builder/test/07-repeated-field.rs" );
