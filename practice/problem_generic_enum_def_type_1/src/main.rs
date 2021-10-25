use std::rc::Rc;
use std::fmt;

//

fn main()
{

  let p1 = Expr::< (), bool >::val( true );
  // let p1 = Expr::val( true );
  println!( "p1 : {:?}", p1 );

  // let p2 = Expr::other( true );
  // println!( "p2 : {:?}", p2 );

  // let p3 = Expr::< (), bool >::Val( true );
  // println!( "p2 : {:?}", p3 );

  // hasDisplay( 13 );
  // hasDisplay( () );

}

//

// pub fn hasDisplay< Src : fmt::Display >( src : Src )
// {
//   println!( "src : {}", src );
// }

//

#[derive( Debug )]
pub enum Expr< T = (), V : Copy = () >
{
  Val( V ),
  Other( Rc< T > ),
}

//

impl< T, V : Copy > Expr< T, V >
{

  pub fn val( val : V ) -> Expr::< (), V >
  {
    Expr::Val( val )
  }

  // pub fn other< T >( p : T ) -> Expr::< T, () >
  // {
  //   Expr::Other( Rc::new( p ) )
  // }

}
