
fn main()
{

  /*
  use macros panic or its analog to terminate the application after fatal error
  */

  // ordinary mean to reach termination
  // < panic!( "Panic" );
  // ! thread 'main' panicked at 'Panic', src/main.rs:4:3

  // on posix run `export set RUST_BACKTRACE=1 && cargo run` to see backtrace of stack
  // on windows run `RUST_BACKTRACE=1 && cargo run` to see backtrace of stack
  panicking_routine();
  // ! thread 'main' panicked at 'Panic', src/main.rs:4:3

  // < unimplemented!( "Unimplemented" );
  // !thread 'main' panicked at 'not implemented: Unimplemented', src/main.rs:7:3

  // < unreachable!( "Unreachable" );
  // !thread 'main' panicked at 'internal error: entered unreachable code: Unreachable', src/main.rs:10:3

}

//

fn panicking_routine()
{
  panic!( "Panic" );
}
