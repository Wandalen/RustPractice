//
// pub fn solution()
// {
//
//   // let mut renderer = Render::new( "solition2".to_string() );
//   // renderer.f_on_update = Box::new( | e | println!( "event from {}", e.renderer.name ) );
//   // renderer.update();
//
// }
//
// //
//
// struct UpdateEvent<'a, OnUpdate>
// where
//   OnUpdate : Fn( &mut Self ),
// {
//   dt : f64,
//   renderer : &'a Render< OnUpdate >,
// }
//
// //
//
// struct Render< OnUpdate >
// where
//   OnUpdate : Fn( &mut UpdateEvent< OnUpdate > ),
// {
//   name : String,
//   f_on_update : dyn Fn( &mut UpdateEvent< OnUpdate > ),
// }
//
// //
//
// impl< OnUpdate > Render< OnUpdate >
// where
//   OnUpdate : Fn( &mut UpdateEvent< OnUpdate > ),
// {
//
//   // fn new( name : String ) -> Render< OnUpdate >
//   fn new( name : String ) -> Self
//   {
//     let f_on_update = | _e : &mut UpdateEvent | {};
//     Self { name, f_on_update : Box::new( f_on_update ) }
//   }
//
//   // fn update( &self )
//   // {
//   //   let mut e = UpdateEvent { dt : 1.0, renderer : self };
//   //   (self.f_on_update)( &mut e );
//   // }
//
// }
