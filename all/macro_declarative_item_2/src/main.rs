//
// In declarative macro after interpreting steam of `tt` as `item` it holds as single `tt` if it is passed further to another macro.
//
// In [this example](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=ef57aebdc6713ef6c44a2a11a621513e) you can see `token!` printed twice. That is explained by hypotheses made above. If `item` consisted of multiple `tt` there would be much more `token!` on the screen.
//
// Is that possible to cancel reinterpretation as `item` and get original stream of `tt` somehow?
//
// [Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=ef57aebdc6713ef6c44a2a11a621513e)
//
// Probably unsolvable problem.
//

macro_rules! impls
{

  () => {};

  (
    $( #[ $Meta : meta ] )*
    $Vis : vis
    fn $Name : ident
    $( $Rest : tt )*
  )
  =>
  {
    impls!
    {
      @DEFINE_FN
      @META{ $( #[ $Meta ] )* }
      @VIS{ $Vis }
      @NAME{ $Name }
      @REST
        $( #[ $Meta ] )*
        $Vis fn $Name
        $( $Rest )*
    }
  };

  (
    @DEFINE_FN
    @META{ $( #[ $Meta : meta ] )* }
    @VIS{ $Vis : vis }
    @NAME{ $Name : ident }
    @REST
      $Item : item
      $( $Rest : tt )*
  )
  =>
  {
    count_tt!( $Item );
    impls!
    {
      $( $Rest )*
    }
  };

}

macro_rules! macro_ignore
{

  (
    $( $Rest : tt )*
  )
  =>
  {
  };

}

macro_rules! count_tt
{
  (
    $( $Tts : tt )*
  )
  =>
  {
    $( println!( "token!" ); macro_ignore!( $Tts ) )*
  };
}

fn main()
{

  impls!
  {
    fn f1()
    {
      println!( "f1" );
    }
    fn f2()
    {
      println!( "f1" );
    }
  }
}
