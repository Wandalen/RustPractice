fn main()
{
  f2( f1 );
}

//

fn f1( src : &str ) -> &str
{
  src
}

//

fn f2< F >( f : F )
where
  F : for< 'a > Fn( &'a str ) -> &'a str,
{
  println!( "{}", f( "abc" ) )
}
