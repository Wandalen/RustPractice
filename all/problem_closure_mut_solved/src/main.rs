// #![ feature( type_name_of_val ) ]
#![ allow( unused_variables ) ]
#![ allow( dead_code ) ]

fn main()
{

  let mut d = 1.0;
  let next_closure = move | o : &mut Opts | -> f32
  {
    d += 1.0;
    o.a += d;
    o.a
  };

  // run( &mut next ); /* that does not work */
  run( next );
  // run( next_closure );

}

//

pub struct Opts
{
  pub a : f32,
}

//

pub fn run< F >
(
  // on_next : &'_ mut F,
  mut on_next : F, /* having referance instead of template does not work */
)
where
  F : FnMut( &mut Opts ) -> f32 + std::marker::Send + 'static,
{
  let mut opts = Opts { a : 13.0 };
  build
  (
    // move | _ : &mut Opts | write( &mut opts, &mut next ),
    // move | _ : &mut Opts | write( &mut opts, on_next ),
    move | _ : &mut Opts | write( &mut opts, &mut on_next ),
  );
}

//

fn build< F >
(
  mut callback : F,
)
where
  F : FnMut( &mut Opts ) + Send + 'static,
{
  let mut opts2 = Opts { a : 3.0 };
  callback( &mut opts2 );
}

//

fn next( o : &mut Opts ) -> f32
{
  o.a += 1.0;
  o.a
}

//

fn write< F >
(
  opts : &mut Opts,
  callback : &mut F,
)
where
  F : ( FnMut( &mut Opts ) -> f32 ) + std::marker::Send + 'static,
{
  callback( opts );
}
