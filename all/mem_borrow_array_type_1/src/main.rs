use std::any::type_name;

fn main()
{
  let array = [ 1, 2, 3, 4, 5 ];
  println!( "array : {} : {:?}", type_name_of( array ), array );
  let slice = &array[ .. ];
  println!( "slice : {} : {:?}", type_name_of( slice ), slice );
}

//

fn type_name_of< T >( _ : &T ) -> &'static str
{
  type_name::< T >()
}
