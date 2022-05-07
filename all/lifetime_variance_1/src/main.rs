fn main()
{

  println!( "Hello, world!" );

}

//

fn strtok1< 'a >( src : &'a mut &'a str, del : char ) -> &'a str
{
  if let Some( i ) = src.find( del )
  {
    let prefix = &src[ .. i ];
    let postfix = &src[ i + del.len_utf8() .. ];
    *src = postfix;
    prefix
  }
  else
  {
    *src = "";
    ""
  }
}

//

fn strtok2< 'a >( src : &'a mut &'a str, del : char )
{
  if let Some( i ) = src.find( del )
  {
    let _prefix = &src[ .. i ];
    let postfix = &src[ i + del.len_utf8() .. ];
    *src = postfix;
    // prefix
  }
  else
  {
    *src = "";
    // ""
  }
}

//

fn strtok1_working< 'a, 'b >( src : &'a mut &'b str, del : char ) -> &'b str
{
  if let Some( i ) = src.find( del )
  {
    let prefix = &src[ .. i ];
    let postfix = &src[ i + del.len_utf8() .. ];
    *src = postfix;
    prefix
  }
  else
  {
    *src = "";
    ""
  }
}

//

fn strtok2_working< 'a, 'b >( src : &'a mut &'b str, del : char )
{
  if let Some( i ) = src.find( del )
  {
    let _prefix = &src[ .. i ];
    let postfix = &src[ i + del.len_utf8() .. ];
    *src = postfix;
    // prefix
  }
  else
  {
    *src = "";
    // ""
  }
}

//

fn make_static< T : Sized >( _ : &'static T )
{
}

//

#[test]
fn basic_test()
{

  // let mut src = "hello world";
  // let got = strtok( &mut src, ' ' );
  // assert_eq!( got, "hello" );
  // assert_eq!( src, "world" );

  let mut src = "hello world";
  // make_static( &mut src );
  let got = strtok2( &mut src, ' ' );
  assert_eq!( got, () );
  // assert_eq!( src, "world" );

}