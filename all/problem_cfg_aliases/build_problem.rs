
// #![ warn( rust_2018_idioms ) ]
// #![ warn( missing_debug_implementations ) ]
// #![ warn( missing_docs ) ]

#![ feature( trace_macros ) ]
#![ recursion_limit = "512" ]

use cfg_aliases::cfg_aliases;

fn main()
{

  trace_macros!( true );
  cfg_aliases!
  {
    // platform
    wasm : { target_arch = "wasm32" },
    android : { target_os = "android" },
    macos : { target_os = "macos" },
    linux : { target_os = "linux" },
    // etc
    nalgebra_ops_x : { feature = "nalgebra_ops" }
    cgmath_ops :
    {
      // all
      // (
        not( nalgebra_ops_x )
        // not( all( feature = "default_ops", feature = "nalgebra" ) ),
        // any( feature = "default_ops", feature = "cgmath_ops" ),
      // )
    },
  }
  trace_macros!( false );

}
