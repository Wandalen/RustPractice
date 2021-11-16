use core::fmt::Display;

fn main()
{
}

//

pub struct VectorFormer< ContainerEnd >
where
  ContainerEnd : Display,
{
  on_end : ContainerEnd,
}

//

impl< ContainerEnd > VectorFormer< ContainerEnd >
where
  ContainerEnd : Display,
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

  pub fn problem< ContainerEnd >() -> VectorFormer< ContainerEnd >
  where
    ContainerEnd : Display
  {
    let on_end = 13;
    VectorFormer::< ContainerEnd >::new( on_end )
  }

}
