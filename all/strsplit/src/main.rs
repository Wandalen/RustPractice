#![ warn( missing_debug_implementations ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_docs ) ]

//!
//! Learning lifetimes
//!

fn main()
{

  let src = "a b c";
  for e in StringSplit::new( src, " " )
  {
    println!( "{:?}", e );
  }

}

//
// Struct
//

#[ derive( Debug ) ]
#[ derive( PartialEq ) ]
pub struct StringSplit< 'a >
{
  src : &'a str,
  del : &'a str,
}

impl< 'a > StringSplit< 'a >
{

  pub fn new( src : &'a str, del : &'a str ) -> Self
  {
    Self { src, del }
  }

}

impl< 'a > Iterator for StringSplit< 'a >
{
  type Item = &'a str;

  fn next( &mut self ) -> Option< Self::Item >
  {
    if let Some( next_del ) = self.src.find( self.del )
    {
      let found = &self.src[ .. next_del ];
      self.src = &self.src[ next_del + self.del.len() .. ];
      Some( found )
    }
    else
    {
      if self.src.is_empty()
      {
        None
      }
      else
      {
        let rest = self.src;
        self.src = "";
        Some( rest )
      }
    }
  }

}

//

#[test]
fn basic_test()
{
  let src = "a b c";
  let letters = StringSplit::new( src, " " );
  assert!( letters.eq( vec!( "a", "b", "c" ).into_iter() ) );

  let src = "a b c";
  for e in StringSplit::new( src, " " )
  {
    println!( "{:?}", e );
  }

}