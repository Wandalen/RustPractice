// https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=4b7ae7def867dac6a03ea5a502544f05

#![ feature( type_name_of_val ) ]

fn main()
{
  let src1 : &[ i32 ] = &[ 1, 2, 3 ];
  dbg!( &std::any::type_name_of_val( src1 ) );
  dbg!( std::mem::size_of_val( &src1 ) );

  let src2 = &[ 1, 2, 3 ];
  dbg!( &std::any::type_name_of_val( src2 ) );
  dbg!( std::mem::size_of_val( &src2 ) );

  dbg!( std::mem::size_of_val( &1_usize ) );
}
