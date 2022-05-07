fn main()
{
  f1();
}

fn f1() -> Option< i32 >
{

  let optional = Some( 13 );

  let x = optional?;

  println!( "{}", x );

  None
}
