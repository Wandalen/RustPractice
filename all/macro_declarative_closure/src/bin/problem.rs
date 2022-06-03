macro_rules! closure
{
  () =>
  {
    macro_rules! macro1
    {
      ( $( $Arg : tt )* ) => { };
    }
  }
}

closure!();

fn main()
{
}
