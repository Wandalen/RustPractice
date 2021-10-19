use std::fmt::Display;
use std::any::type_name;

fn main()
{

  str_log1( "slice1" );
  str_log1( "string1".to_string() );

  str_log2( "slice2" );
  str_log2( "string2".to_string() );

}

//

fn str_log1< Src >( src : Src )
where
  Src : Display + Into< String >
{
  println!( "{} : {}", src, type_name_of( &src ) );
}

//

fn str_log2( src : impl Display + Into< String > )
{
  println!( "{} : {}", src, type_name_of( &src ) );
}

//

fn type_name_of< T >( _ : &T ) -> &'static str
{
  type_name::< T >()
}
