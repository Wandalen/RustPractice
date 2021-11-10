#![allow(dead_code)]

use std::time::Duration;

#[derive(Debug)]
struct Record< 'a >
{
  input : String,
  lines : Vec< &'a str >,
}

fn main()
{

  let input = "I am good line\ni am hiding\n\nI will be selected\ni will not\n".to_string();
  let input_slice = unsafe
  {
    std::str::from_utf8_unchecked( std::slice::from_raw_parts( input.as_ptr(), input.len() ) )
  };

  let lines : Vec<_> = input_slice.lines().filter( | line | starts_with_capital( line ) ).collect();
  dbg!( &lines );

  let record = Record
  {
    input,
    lines,
  };

  let handle = std::thread::spawn( move ||
  {
    for line in &record.lines
    {
      std::thread::sleep( Duration::from_millis( 500 ) );
      println!( "{}", line );
    }
    dbg!( &record );
  });

  handle.join().unwrap();
}

//

fn starts_with_capital( s : &str ) -> bool
{
  // println!( "{:?}", s.chars() );
  matches!( s.chars().next(), Some( c ) if c.is_uppercase() )
}
