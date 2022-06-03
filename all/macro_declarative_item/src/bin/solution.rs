macro_rules! that_is_item
{

  (
    @STAGE2
    $( $Token : tt )*
  )
  =>
  {
  };

  (
    $( $Item : item )+
  )
  =>
  {
    $( that_is_item!( @STAGE2 $Item ) )+;
  };

}


fn main()
{
  that_is_item!
  {
    fn f1() -> i32
    {
      13
    }
  };
}
