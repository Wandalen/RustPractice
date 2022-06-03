fn main()
{
  let src = [ 1, 2, 3 ];
  let mut dst = [ 3, 4, 5 ];
  copy( &src[ .. ], &mut dst[ .. ] );
  dbg!( &src );
  dbg!( &dst );
}

//

fn copy< 'a, 'b, T >( src : &'a [ T ], dst : &'b mut [ T ] )
where
  T : Copy,
{
  assert!( src.len() == dst.len() );
  dst.copy_from_slice( src );
  // *dst = *src;
}
