
// fn main()
// {

//   use ::core::marker::PhantomData;
//   trait Trait1 {}
//   impl< T : Sized > Trait1 for &[ T ] {}

//   trait True
//   {
//     fn get( self : &'_ Self ) -> bool { true }
//   }
//   // impl< T > True for PhantomData< &'_ [ T ] >
//   // where
//   //   T : Trait1 + Sized,
//   // {}

//   impl< 'a, T > True for PhantomData< &'a [ T ] >
//   where
//     // T : Trait1 + Sized,
//     &'a [ T ] : Trait1
//   {}

//   fn does< T : Sized >( _ : &[ T ] ) -> PhantomData< &[ T ] >
//   {
//     PhantomData
//   }

//   let got = ( &does( &[ 1, 2, 3 ] ) ).get();
//   assert_eq!( got, true );

// }

//

// #[ macro_export ]
// macro_rules! does_impl
// {
//   ( $V : expr => $( $Traits : tt )+ ) =>
//   {{
//     use ::core::marker::PhantomData;

//     trait False
//     {
//       fn get( self : &'_ Self ) -> bool { false }
//     }

//     impl< T > False
//     for &'_ PhantomData< T >
//     where T : ?Sized,
//     {}

//     trait True
//     {
//       fn get( self : &'_ Self ) -> bool { true }
//     }

//     impl< T > True for PhantomData< T >
//     where T : $( $Traits )+ + ?Sized,
//     {}

//     // impl< 'a, T > True for PhantomData< &'a [ T ] >
//     // where &'a [ T ] : $( $Traits )+,
//     // {}

//     fn does< T : ?Sized >( _ : &T ) -> PhantomData< T >
//     {
//       PhantomData
//     }

//     // fn does< T : Sized >( _ : &T ) -> PhantomData< &[ T ] >
//     // {
//     //   PhantomData
//     // }

//     ( &does( &$V ) ).get()

//   }}
// }

//

// #[ macro_export ]
// macro_rules! does_impl
// {
//   ( $V : expr => $( $Traits : tt )+ ) =>
//   {{
//     use ::core::marker::PhantomData;

//     trait False
//     {
//       fn get( self : &'_ Self ) -> bool { false }
//     }

//     impl< T > False
//     for &'_ PhantomData< T >
//     where T : ?Sized,
//     {}

//     // impl< T > False
//     // for PhantomData< T >
//     // where T : ?Sized,
//     // {}

//     trait True
//     {
//       fn get( self : &'_ Self ) -> bool { true }
//     }

//     impl< T > True for PhantomData< T >
//     where T : $( $Traits )+ + ?Sized,
//     {}

//     // impl< 'a, T > True for PhantomData< &'a [ T ] >
//     // where &'a [ T ] : $( $Traits )+,
//     // {}

//     trait Slice< T : Sized >
//     {
//       fn does( &self ) -> PhantomData< &[ T ] >
//       // where Self : Sized,
//       {
//         println!( "Slice::does" );
//         PhantomData
//       }
//     }

//     impl< 'a, T > Slice< &[ T ] >
//     for &'a [ T ]
//     where T : Sized,
//     {}

//     // impl< T > Slice< T >
//     // for [ T ]
//     // where T : Sized,
//     // {}

//     trait NotSlice
//     {
//       fn does( self : &'_ Self ) -> PhantomData< Self >
//       {
//         println!( "NotSlice::does" );
//         PhantomData
//       }
//     }

//     impl< T > NotSlice
//     for &T
//     // where T : ?Sized,
//     {}

//     ( &( ( &$V ).does() ) ).get()
//     // ( &$V ).does();
//     // true

//     // fn does< T : Sized >( _ : &T ) -> PhantomData< T >
//     // {
//     //   PhantomData
//     // }
//     // ( &does( &$V ) ).get()

//   }}
// }

// working for most cases!

// #[ macro_export ]
// macro_rules! does_impl
// {
//   ( $V : expr => $( $Traits : tt )+ ) =>
//   {{
//     use ::core::marker::PhantomData;

//     trait False
//     {
//       fn get( self : &'_ Self ) -> bool { false }
//     }

//     impl< T > False
//     for &'_ PhantomData< T >
//     where T : ?Sized,
//     {}

//     trait True
//     {
//       fn get( self : &'_ Self ) -> bool { true }
//     }

//     impl< T > True for PhantomData< T >
//     where T : $( $Traits )+ + ?Sized,
//     {}

//     fn does< T : Sized >( _ : &T ) -> PhantomData< T >
//     {
//       PhantomData
//     }
//     ( &does( &$V ) ).get()

//   }}
// }

// //

// #[ macro_export ]
// macro_rules! does_impl
// {
//   ( $V : expr => $( $Traits : tt )+ ) =>
//   {{
//     use ::core::marker::PhantomData;

//     trait False
//     {
//       fn get( self : &'_ Self ) -> bool { false }
//     }

//     impl< T > False
//     for &'_ PhantomData< T >
//     where T : ?Sized,
//     {}

//     // impl< T > False
//     // for PhantomData< T >
//     // where T : ?Sized,
//     // {}

//     trait True
//     {
//       fn get( self : &'_ Self ) -> bool { true }
//     }

//     impl< T > True for PhantomData< T >
//     where T : $( $Traits )+ + ?Sized,
//     {}

//     // impl< T > True for PhantomData< &T >
//     // where T : $( $Traits )+ + ?Sized,
//     // {}

//     // impl< T > True for PhantomData< [ T ] >
//     // where T : $( $Traits )+ + Sized,
//     // {}

//     // impl< 'a, T > True for PhantomData< &'a [ T ] >
//     // where &'a [ T ] : $( $Traits )+,
//     // {}

//     // trait Slice< T : Sized >
//     // {
//     //   fn does( &self ) -> PhantomData< &[ T ] >
//     //   // where Self : Sized,
//     //   {
//     //     println!( "Slice::does" );
//     //     PhantomData
//     //   }
//     // }

//     // impl< 'a, T > Slice< &[ T ] >
//     // for &'a [ T ]
//     // where T : Sized,
//     // {}

//     // // impl< T > Slice< T >
//     // // for [ T ]
//     // // where T : Sized,
//     // // {}

//     // trait NotSlice
//     // {
//     //   fn does( self : &'_ Self ) -> PhantomData< Self >
//     //   {
//     //     println!( "NotSlice::does" );
//     //     PhantomData
//     //   }
//     // }

//     // impl< T > NotSlice
//     // for &T
//     // // where T : ?Sized,
//     // {}

//     // ( &( ( &$V ).does() ) ).get()
//     // ( &$V ).does();
//     // true

//     fn does< T : Sized >( _ : &T ) -> PhantomData< T >
//     {
//       PhantomData
//     }
//     ( &does( &$V ) ).get()

//     // trait NotSlice
//     // {
//     //   fn is_slice( self : &'_ Self ) -> bool { false }
//     // }

//     // impl< T > NotSlice
//     // for &'_ PhantomData< T >
//     // where T : ?Sized,
//     // {}

//     // trait Slice
//     // {
//     //   fn is_slice( self : &'_ Self ) -> bool { true }
//     // }

//     // impl< 'a, T > Slice for PhantomData< &'a &[ T ] >
//     // {}

//     // fn does< T : Sized >( _ : &T ) -> PhantomData< &T >
//     // {
//     //   PhantomData
//     // }

//     // ( &does( &$V ) ).is_slice()

//   }}
// }

//

#[ macro_export ]
macro_rules! is_slice
{
  ( $V : expr ) =>
  {{
    use ::core::marker::PhantomData;

    trait NotSlice
    {
      fn is_slice( self : &'_ Self ) -> bool { false }
    }

    impl< T > NotSlice
    for &'_ PhantomData< T >
    where T : ?Sized,
    {}

    trait Slice
    {
      fn is_slice( self : &'_ Self ) -> bool { true }
    }

    impl< 'a, T > Slice for PhantomData< &'a &[ T ] >
    {}

    fn does< T : Sized >( _ : &T ) -> PhantomData< &T >
    {
      PhantomData
    }

    ( &does( &$V ) ).is_slice()

  }}
}

//

#[ macro_export ]
macro_rules! does_impl
{
  ( $V : expr => $( $Traits : tt )+ ) =>
  {{
    use ::core::marker::PhantomData;

    trait False
    {
      fn get( self : &'_ Self ) -> bool { false }
    }

    impl< T > False
    for &'_ PhantomData< T >
    where T : ?Sized,
    {}

    trait True
    {
      fn get( self : &'_ Self ) -> bool { true }
    }

    impl< T > True for PhantomData< T >
    where T : $( $Traits )+ + ?Sized,
    {}

    fn does< T : Sized >( _ : &T ) -> PhantomData< T >
    {
      PhantomData
    }
    ( &does( &$V ) ).get()

  }}
}

//

fn main()
{

  let src : &[ i32 ] = &[ 1, 2, 3 ];
  assert_eq!( is_slice!( src ), true );
  assert_eq!( is_slice!( vec!( 1, 2, 3 ) ), false );
  assert_eq!( is_slice!( 13_f32 ), false );
  assert_eq!( is_slice!( true ), false );
  let src = false;
  assert_eq!( is_slice!( src ), false );
  assert_eq!( is_slice!( Box::new( true ) ), false );
  let src = Box::new( true );
  assert_eq!( is_slice!( src ), false );

  trait Trait1 {}
  fn does_implement_trait1( _ : &impl Trait1 ) -> bool { true }

  impl< T : Sized > Trait1 for &[ T ] {}
  impl< T : Sized, const N : usize > Trait1 for [ T; N ] {}
  impl< T : Sized, const N : usize > Trait1 for &[ T; N ] {}
  let src : &[ i32 ] = &[ 1, 2, 3 ];
  assert_eq!( does_impl!( src => Trait1 ), true );
  assert_eq!( does_implement_trait1( &src ), true );
  assert_eq!( does_impl!( &[ 1, 2, 3 ] => Trait1 ), true );
  assert_eq!( does_implement_trait1( &[ 1, 2, 3 ] ), true );
  assert_eq!( does_impl!( [ 1, 2, 3 ] => Trait1 ), true );

  impl< T : Sized > Trait1 for Vec< T > {}
  assert_eq!( does_impl!( vec!( 1, 2, 3 ) => Trait1 ), true );

  impl Trait1 for f32 {}
  assert_eq!( does_impl!( 13_f32 => Trait1 ), true );

  assert_eq!( does_impl!( true => Copy ), true );
  assert_eq!( does_impl!( true => Clone ), true );

  let src = true;
  assert_eq!( does_impl!( src => Copy ), true );
  assert_eq!( does_impl!( src => Clone ), true );

  assert_eq!( does_impl!( Box::new( true ) => Copy ), false );
  assert_eq!( does_impl!( Box::new( true ) => Clone ), true );

  let src = Box::new( true );
  assert_eq!( does_impl!( src => Copy ), false );
  assert_eq!( does_impl!( src => Clone ), true );

  let pointer_size = std::mem::size_of::< &u8 >();
  dbg!( &pointer_size );
  assert_eq!( 2 * pointer_size, std::mem::size_of::< &[ u8 ] >() );
  assert_eq!( 2 * pointer_size, std::mem::size_of::< *const [ u8 ] >() );
  assert_eq!( 2 * pointer_size, std::mem::size_of::< Box< [ u8 ] > >() );
  assert_eq!( 2 * pointer_size, std::mem::size_of::< std::rc::Rc< [ u8 ] > >() );
  assert_eq!( 1 * pointer_size, std::mem::size_of::< &[ u8 ; 20 ] >() );

}
