#![ feature( type_name_of_val ) ]

use std::io;
use io::prelude::*;
use io::SeekFrom;
use io::Cursor;

mod wt;

//

fn main()
{

  vector_write();
  slice_write();
  vector_extand();
  slice_extend();
  repeat();

}

//

fn vector_write()
{
  println!( "== vector_write\n" );

  let container1 = vec![ 0; 15 ];
  println!( "{:p}, {:?}", &container1[ .. ], &container1 );
  let mut buff = Cursor::new( container1 );

  write_ten_bytes_at_end( &mut buff ).unwrap();
  assert_eq!( &buff.get_ref()[ 5..15 ], &[ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 ] );

  let container2 = &buff.into_inner();
  inspect_logging_type_of!( container2 );
  println!( "{:p} {:?}", &container2[ .. ], container2 );

}

//

fn slice_write()
{
  println!( "== slice_write\n" );

  let mut container1 = [ 0; 15 ];
  println!( "{:p}, {:?}", &container1[ .. ], &container1 );
  let mut buff = Cursor::new( &mut container1[ .. ] );

  write_ten_bytes_at_end( &mut buff ).unwrap();
  assert_eq!( &buff.get_ref()[ 5..15 ], &[ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 ] );

  let container2 = &buff.into_inner();
  inspect_logging_type_of!( container2 );
  println!( "{:p} {:?}", &container2[ .. ], container2 );

}

//

fn vector_extand()
{
  println!( "== vector_extand\n" );

  let container1 = vec![ 0; 2 ];
  println!( "{:p}, {:?}", &container1[ .. ], &container1 );
  let mut buff = Cursor::new( container1 );

  buff.write( &[ 1, 2, 3, 4, 5, 6, 7, 8 ] ).unwrap();
  assert_eq!( &buff.get_ref()[ .. ], &[ 1, 2, 3, 4, 5, 6, 7, 8 ] );

  let container2 = &buff.into_inner();
  inspect_logging_type_of!( container2 );
  println!( "{:p} {:?}", &container2[ .. ], container2 );

}

//

fn slice_extend()
{
  println!( "== slice_extend\n" );

  let mut container1 = [ 0; 2 ];
  println!( "{:p}, {:?}", &container1[ .. ], &container1 );
  let mut buff = Cursor::new( &mut container1[ .. ] );

  buff.write( &[ 1, 2, 3, 4, 5, 6, 7, 8 ] ).unwrap();
  assert_eq!( &buff.get_ref()[ .. ], &[ 1, 2 ] );

  let container2 = &buff.into_inner();
  inspect_logging_type_of!( container2 );
  println!( "{:p} {:?}", &container2[ .. ], container2 );

}

//

fn repeat()
{
  println!( "== repeat\n" );

  let mut container1 = vec![ 0; 2 ];
  println!( "{:p}, {:?}", &container1[ .. ], &container1 );
  let mut buff = Cursor::new( &mut container1 );

  io::copy( &mut io::repeat( 13 ).take( 5 ), &mut buff ).unwrap();
  // io::repeat( 13 ).chain( &buff ); // buff.write(  ).unwrap();
  assert_eq!( &buff.get_ref()[ .. ], &[ 13, 13, 13, 13, 13 ] );

  let container2 = &buff.into_inner();
  inspect_logging_type_of!( container2 );
  println!( "{:p} {:?}", &container2[ .. ], container2 );

}

//

fn write_ten_bytes_at_end< W : Write + Seek >( writer : &mut W ) -> io::Result<()>
{
  writer.seek( SeekFrom::End( -10 ) )?;
  for i in 0..10
  {
    writer.write( &[ i ] )?;
  }
  // all went well
  Ok(())
}
