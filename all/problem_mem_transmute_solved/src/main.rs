use std::borrow::Borrow;

fn main()
{
  let header = ObjectHeader
  {
    x : 1,
    h : 2,
    a : 3,
    f : 4,
    w : 5,
    body : (),
  };

  let object = header.form();
  let borrowed_header : &ObjectHeader = object.borrow();
  dbg!( borrowed_header );

}

//

#[ repr( C ) ] - /* solve the problem */
#[ derive( Debug, Clone ) ]
pub struct ObjectGeneric< Body >
{
  pub x : i64,
  pub h : u32,
  pub a : u8,
  pub f : u16,
  pub w : u32,
  pub body : Body,
}

pub type ObjectHeader = ObjectGeneric< () >;

impl ObjectHeader
{
  pub fn form( self ) -> Object
  {
    let body = ObjectBody { j : 10 };
    Object
    {
      x : self.x,
      h : self.h,
      a : self.a,
      f : self.f,
      w : self.w,
      body,
    }
  }
}

#[ derive( Debug, Clone ) ]
pub struct ObjectBody
{
  pub j : u32,
}

pub type Object = ObjectGeneric< ObjectBody >;

//

impl Borrow< ObjectHeader > for Object
{
  fn borrow<'a>( &'a self ) -> &'a ObjectHeader
  {
    unsafe
    {
      dbg!( &self );
      let result = std::mem::transmute::< &'a Object, &'a ObjectHeader >( self );
      dbg!( &result );
      result
    }
  }
}

//
