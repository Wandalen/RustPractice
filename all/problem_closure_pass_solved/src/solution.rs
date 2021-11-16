#![allow(dead_code)]

fn main()
{
}

//

pub struct VectorFormer< ContainerEnd >
where
  ContainerEnd : Fn(),
{
  on_end : ContainerEnd,
}

//

impl< ContainerEnd > VectorFormer< ContainerEnd >
where
  ContainerEnd : Fn(),
{
  fn new( on_end : ContainerEnd ) -> Self
  {
    Self { on_end }
  }
}

//

pub struct CommandFormer
{
}

//

impl CommandFormer
{

  pub fn problem() -> VectorFormer< impl Fn() >
  {
    let on_end = ||
    {
      println!( "on_end" );
    };
    VectorFormer::new( on_end )
  }

}
