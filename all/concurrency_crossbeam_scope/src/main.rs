use std::time::Duration;

fn main()
{

  let input = "I am good line\ni am hiding\n\nI will be selected\ni will not\n";
  // println!( "{}", input );

  // for line in input.lines().filter( | line | starts_with_capital( line ) )
  // {
  //   println!( "{}", line );
  // }

  crossbeam::scope( | scope |
  {
    scope.spawn( | _ |
    {

      let lines : Vec<_> = input.lines().filter( | line | starts_with_capital( line ) ).collect();
      dbg!( &lines );

      for line in lines
      {
        std::thread::sleep( Duration::from_millis( 500 ) );
        println!( "{}", line );
      }

    });
  }).unwrap();

  // let handle = std::thread::spawn( move ||
  // {
  //   for line in lines
  //   {
  //     std::thread::sleep( Duration::from_millis( 500 ) );
  //     println!( "{}", line );
  //   }

  // });

  // handle.join().unwrap();
}

//

fn starts_with_capital( s : &str ) -> bool
{
  // println!( "{:?}", s.chars() );
  matches!( s.chars().next(), Some( c ) if c.is_uppercase() )
}
