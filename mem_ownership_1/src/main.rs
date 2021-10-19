fn main()
{

  borrow_to_read_demo();
  borrow_to_routine_demo();
  drop_demo();

}

//

/* related links :
- https://www.youtube.com/watch?v=2IxQgXQl_Ws
- https://www.youtube.com/watch?v=VFIOSWy93H0
- https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
*/

/* Rules of ownership
1. Each value has a variable which is its owner.
2. There can only be one owner at any given time.
3. When the owner goes out of scope, the value will be dropped out of memory.
*/

/* Rules of borrowing
1. Allowed infinity borrows for readonly access.
2. Readonly borrows make the original data become immutable for durations of borrows.
3. Only allowed to pass one borrow at a time to write.
*/

//

fn borrow_to_read_demo()
{
  println!( "borrow_to_read_demo" );
  let mut string1 = "string2".to_string();
  let string2 = &string1;
  let string3 = &string1;
  println!( "string1 : {}", string1 );
  println!( "string2 : {}", string2 );
  // < string1.push( '!' );
  // ! cannot borrow `string1` as mutable because it is also borrowed as immutable
  println!( "string3 : {}", string3 );
  string1.push( '!' );
  println!( "" );
}

//

fn borrow_to_routine_demo()
{
  println!( "borrow_to_routine_demo" );
  let string1 = "string1".to_string();
  println!( "string1 : {}", string1 );
  own_string( string1 );
  // println!( "string1 : {}", string1 );
  // ! borrow of moved value: `string1`
  println!( "" );
}

//

fn own_string( string : String )
{
  println!( "own_string got ownership of {}", string );
}

//

fn drop_demo()
{
  let string1 = "abc".to_string();
  drop( string1 );
  // < println!( "string1 : {}", string1 );
  // ! borrow of moved value: `string1`
}

//

fn drop< T >( _ : T )
{
}