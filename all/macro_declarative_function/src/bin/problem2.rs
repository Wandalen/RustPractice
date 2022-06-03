
// no solution

fn main()
{
}

macro_rules! parse_fn
{

  () => {};

  (
    $( #[ $Meta : meta ] )*
    $Vis : vis
    fn $Name : ident
    $( < $( $ParamName : ident : $ParamType : ty ),* > )?
    $Input : tt
    $( $Rest : tt )*
  )
  =>
  {
    parse_fn!
    {
      as DEFINE_FN
      as META $( #[ $Meta ] )*
      as VIS $Vis
      as NAME $Name
      as INPUT $Input
      as OUTPUT
      as BLOCK {}
      as REST
        $( #[ $Meta ] )*
        $Vis fn $Name $( < $( $ParamName : $ParamType ),* > )?
        $Input
        $( $Rest )*
    }
  };

  (
    as DEFINE_FN
    as META $( #[ $Meta : meta ] )*
    as VIS $Vis : vis
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
    parse_fn!
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

  fn f3< T : Clone >( _ : T )
  {
    println!( "f3" );
  }

}
