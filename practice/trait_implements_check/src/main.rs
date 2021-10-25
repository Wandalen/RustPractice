
// use ::core::marker::PhantomData;

macro_rules! does
{
  ( $V : ident impl $( $bounds : tt )* ) =>
  {{
    use ::core::marker::PhantomData;

    trait DoesDefault
    {
      fn value( self : &'_ Self ) -> bool { false }
    }

    impl< T > DoesDefault
    for &'_ PhantomData< T >
    where T : ?Sized,
    {}

    trait DoesTrue
    {
      fn value( self : &'_ Self ) -> bool { true }
    }
    impl< T > DoesTrue for PhantomData<T>
    where
      T : $( $bounds )* + ?Sized,
    {
    }

    fn does< T : ?Sized >( _ : &T ) -> PhantomData<T>
    {
      PhantomData
    }
    ( &does( &$V ) ).value()
  }}
}

//

fn main()
{
  let a = true;
  let b = Box::new( true );
  let a_impl_copy = does!( a impl Copy );
  let b_impl_copy = does!( b impl Copy );
  let b_impl_clone = does!( b impl Clone );
  dbg!( a_impl_copy, b_impl_copy, b_impl_clone );
}


