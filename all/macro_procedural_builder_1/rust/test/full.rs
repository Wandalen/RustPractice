//   - An example of a derive macro implemented using Syn:
//     https://github.com/dtolnay/syn/tree/master/examples/heapsize

#[macro_use] extern crate maplit;
use derive_builder::Builder;

pub trait VectorLike< E >
{
  fn push( &mut self, e : E );
}

impl< E > VectorLike< E > for Vec< E >
{
  fn push( &mut self, e : E )
  {
    Vec::push( self, e );
  }
}

#[derive( Debug, Default )]
pub struct VectorFormer< E, Vector, Former, ContainerEnd >
where
  Vector : VectorLike< E >,
  Former : Default,
  ContainerEnd : Fn( &mut Former, Vector ),
{
  container : Option< Vector >,
  former : Former,
  on_end : ContainerEnd,
  _phantom : core::marker::PhantomData< E >,
  // _phantom2 : core::marker::PhantomData< Former >,
}

impl< E, Vector, Former, ContainerEnd > VectorFormer< E, Vector, Former, ContainerEnd >
where
  Vector : VectorLike< E >,
  Former : Default,
  ContainerEnd : Fn( &mut Former, Vector ),
{
  fn push< E2 >( &mut self, e : E2 ) -> &mut Self
  where E2 : core::convert::Into< E >,
  {
    // if Some( self.container )
    if let core::option::Option::Some( ref mut container ) = self.container
    {
      container.push( e.into() );
    }
    self
  }

  fn new( former : Former, container : Vector, on_end : ContainerEnd ) -> Self
  {
    // container = Default::default();
    Self
    {
      former,
      container : Some( container ),
      on_end,
      _phantom : core::marker::PhantomData,
      // _phantom2 : core::marker::PhantomData,
    }
  }

  fn end( mut self ) -> Former
  {
    let container = self.container.take().unwrap();
    (self.on_end)( &mut self.former, container );
    self.former
    // Default::default()
  }
}

#[derive( Debug, PartialEq )]
// #[derive( Builder )]
pub struct Command
{
  executable : String,
  args : Vec< String >,
  env : std::collections::HashMap< String, String >,
  current_dir : Option< String >,
}

//

impl Command
{
  pub fn former() -> CommandFormer
  {
    CommandFormer
    {
      executable : core::option::Option::None,
      args : core::option::Option::None,
      env : core::option::Option::None,
      current_dir : core::option::Option::None,
    }
  }
}

//

#[derive( Debug, Default )]
pub struct CommandFormer
{
  executable : core::option::Option< String >,
  // args : core::option::Option< VectorFormer< String, Vec< String >, CommandFormer > >,
  args : core::option::Option< Vec< String > >,
  env : core::option::Option< std::collections::HashMap< String, String > >,
  current_dir : core::option::Option< String >,
}

//

impl CommandFormer
{
  fn form( mut self ) -> Command
  {

    let executable = if let core::option::Option::Some( _val ) = &self.executable
    {
      self.executable.take().unwrap()
    }
    else
    {
      let val : String = Default::default();
      val
    };

    let args = if let core::option::Option::Some( _val ) = &self.args
    {
      self.args.take().unwrap()
    }
    else
    {
      let val : Vec< String > = Default::default();
      val
    };

    let env = if let core::option::Option::Some( _val ) = &self.env
    {
      self.env.take().unwrap()
    }
    else
    {
      let val : std::collections::HashMap< String, String > = Default::default();
      val
    };

    let current_dir = if let core::option::Option::Some( _val ) = &self.current_dir
    {
      Some( self.current_dir.take().unwrap() )
    }
    else
    {
      None
    };

    Command
    {
      executable,
      args,
      env,
      current_dir,
    }

  }

  pub fn executable< Src >( &mut self, src : Src ) -> &mut Self
  where Src : core::convert::Into< String >,
  {
    self.executable = Some( src.into() );
    self
  }

  pub fn args< ContainerEnd >( mut self ) ->
    VectorFormer< String, Vec< String >, CommandFormer, ContainerEnd >
  where
    ContainerEnd : Fn( &mut CommandFormer, Vec< String > )
  {

    if let core::option::Option::Some( ref _val ) = self.args
    {
    }
    else
    {
      self.args = Some( Default::default() );
    }

    if let Some( _val ) = &mut self.args
    {
      let container = self.args.take().unwrap();
      let on_end = | former : &mut CommandFormer, container : Vec< String > |
      {
        former.args = Some( container );
      };
      VectorFormer::< String, Vec< String >, CommandFormer, ContainerEnd >::new( self, container, on_end )
    }
    else
    {
      unreachable!()
    }

  }

  // pub fn args< Src >( &mut self, src : Src ) -> &mut Self
  // where Src : core::convert::Into< Vec< String > >
  // {
  //   self.args = Some( src.into() );
  //   self
  // }

  // pub fn args_append< Src >( &mut self, src : Src ) -> &mut Self
  // where Src : core::convert::Into< String >
  // {
  //   if let core::option::Option::Some( _ ) = &self.args
  //   {
  //   }
  //   else
  //   {
  //     self.args = Some( Default::default() );
  //   };
  //   if let core::option::Option::Some( dst ) = &mut self.args
  //   {
  //     dst.push( src.into() );
  //   }
  //   self
  // }

  pub fn env< Src >( &mut self, src : Src ) -> &mut Self
  where Src : core::convert::Into< std::collections::HashMap< String, String > >
  {
    self.env = Some( src.into() );
    self
  }

  pub fn current_dir< Src >( &mut self, src : Src ) -> &mut Self
  where Src : core::convert::Into< Option< String > >
  {
    self.current_dir = src.into();
    self
  }

}

//

fn main()
{

  // // test.case( "basic" );

  // let mut command_unformed = Command::former();
  // let command = command_unformed
  // .executable( "Abc".to_string() )
  // .executable( "Abcd" )
  // .args( vec![ "a".to_string(), "bc".to_string(), "def".to_string() ] )
  // .form();
  // dbg!( &command );

  // let expected = Command
  // {
  //   executable : "Abcd".to_string(),
  //   args : vec![ "a".to_string(), "bc".to_string(), "def".to_string(), "ghi".to_string(), "klm".to_string() ],
  //   env : hashmap!{},
  //   current_dir : None,
  // };
  // assert_eq!( command, expected );

  // // test.case( "executable : slice" );

  // let mut command_unformed = Command::former();
  // let command = command_unformed
  // .executable( "Abcd" )
  // .form();
  // dbg!( &command );

  // let expected = Command
  // {
  //   executable : "Abcd".to_string(),
  //   args : vec![],
  //   env : hashmap!{},
  //   current_dir : None,
  // };
  // assert_eq!( command, expected );

  // test.case( "args : implicit construction" );

  let mut command_unformed = Command::former();
  let command = command_unformed
  .args().push( "ghi" ).push( "klm" ).end()
  .form();
  dbg!( &command );

  let expected = Command
  {
    executable : "".to_string(),
    args : vec![ "ghi".to_string(), "klm".to_string() ],
    env : hashmap!{},
    current_dir : None,
  };
  assert_eq!( command, expected );

}

//

#[ test ]
fn main_test()
{
  main();
}
