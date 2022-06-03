
fn main()
{
}

macro_rules! parse_fn
{

  () => {};

  (
    $( #[ $Meta : meta ] )*
    $( pub )?
    fn $Name : ident
    $( $Rest : tt )*
  )
  =>
  {
    $crate::parse_fn!
    {
      as DEFINE_FN
      as META $( #[ $Meta ] )*
      as VIS $( pub )?
      as NAME $Name
      as INPUT ()
      as OUTPUT
      as BLOCK {}
      as REST
        $( #[ $Meta ] )*
        $( pub )? fn $Name
        $( $Rest )*
    }
  };

  (
    as DEFINE_FN
    as META $( #[ $Meta : meta ] )*
    as VIS $( pub )*
    as NAME $Name : ident
    as INPUT $Input : tt
    as OUTPUT $( -> $Output : ty )?
    as BLOCK $Block : block
    as REST
      $Item : item
      $( $Rest : tt )*
  )
  =>
  {
    macro_rules! $Name
    {
      () =>
      {
        $Item
      };
    }
    $crate::parse_fn!
    {
      $( $Rest )*
    }
  };

}

parse_fn!
{

  pub fn f1()
  {
    println!( "f1" );
  }

  fn f2()
  {
    println!( "f1" );
  }

}
