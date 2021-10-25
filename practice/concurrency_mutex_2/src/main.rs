use std::sync::Mutex;

fn main()
{

  let book = Book
  {
    name : Mutex::new( "Book1" ),
    author : Mutex::new( "Bad Marg" ),
    nsold : Mutex::new( 13 ),
  };
  println!( "book : {:?}", book );
  // < book : Book { name: Mutex { data: "Book1", poisoned: false, .. }, author: Mutex { data: "Bad Marg", poisoned: false, .. }, nsold: Mutex { data: 13, poisoned: false, .. } }

  *book.nsold.lock().unwrap() += 10;
  println!( "book : {:?}", book );
  // < book : Book { name: Mutex { data: "Book1", poisoned: false, .. }, author: Mutex { data: "Bad Marg", poisoned: false, .. }, nsold: Mutex { data: 23, poisoned: false, .. } }

}

//

#[ derive( Debug ) ]
struct Book< 'a >
{
  name : Mutex< &'a str  >,
  author : Mutex< &'a str  >,
  nsold : Mutex< u32  >,
}