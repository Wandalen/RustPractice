#![allow(dead_code)]

use std::time::Duration;

mod internal
{

  #[derive(Debug)]
  pub struct Record< 'a >
  {
    input : String,
    lines : Vec< &'a str >,
  }

  impl< 'a > Record< 'a >
  {
    pub fn new( input : String ) -> Self
    {
      let input_slice = unsafe
      {
        std::str::from_utf8_unchecked( std::slice::from_raw_parts( input.as_ptr(), input.len() ) )
      };

      let lines : Vec<_> = input_slice.lines().filter( | line | starts_with_capital( line ) ).collect();
      // dbg!( &lines );

      Record
      {
        input,
        lines,
      }
    }
    pub fn lines( &self ) -> &[ &str ]
    {
      &self.lines[ .. ]
    }
  }

  fn starts_with_capital( s : &str ) -> bool
  {
    // println!( "{:?}", s.chars() );
    matches!( s.chars().next(), Some( c ) if c.is_uppercase() )
  }

}

use internal::Record;

//

fn main()
{

  let input = "I am good line\ni am hiding\n\nI will be selected\ni will not\n".to_string();

  let record = Record::new( input );

  let handle = std::thread::spawn( move ||
  {
    for line in record.lines()
    {
      std::thread::sleep( Duration::from_millis( 500 ) );
      println!( "{}", line );
    }
    dbg!( &record );
  });

  handle.join().unwrap();
}
