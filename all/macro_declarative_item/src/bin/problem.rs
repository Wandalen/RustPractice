macro_rules! that_is_item
{
  (
    $( $Item : item )+
  )
  =>
  {
    $( that_is_item!( @STAGE2 $Item ) )+;
  };

  (
    @STAGE2
    $( $Token : tt )*
  )
  =>
  {
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
