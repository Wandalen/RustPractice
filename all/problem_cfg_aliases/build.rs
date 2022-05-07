// #![ warn( rust_2018_idioms ) ]
// #![ warn( missing_debug_implementations ) ]
// #![ warn( missing_docs ) ]

// #![ feature( trace_macros ) ]
// #![ recursion_limit = "512" ]

use cfg_aliases::cfg_aliases;

fn main()
{

  // trace_macros!( true );
  cfg_aliases!
  {
    // etc
    nalgebra_ops_x : { feature = "nalgebra_ops" }
    cgmath_ops :
    {
      not( nalgebra_ops_x )
    },
  }
  // trace_macros!( false );

}
