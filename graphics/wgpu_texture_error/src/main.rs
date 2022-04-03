
fn main()
{

  let ( _event_loop, _window, surface, surface_config, device, queue ) = context_make();
  let render_pipeline = render_pipeline_make( &device, &surface_config );
  render( &device, &surface, &queue, &render_pipeline ).unwrap();

}

//

pub fn render_pipeline_make
(
  device : &wgpu::Device,
  surface_config : &wgpu::SurfaceConfiguration,
)
->
wgpu::RenderPipeline
{

  let shader = device.create_shader_module( &wgpu::ShaderModuleDescriptor
  {
    label : Some( "Daignostics Triangle Shader" ),
    source : wgpu::ShaderSource::Wgsl( SHADER.into() ),
  });

  let render_pipeline = device.create_render_pipeline( &wgpu::RenderPipelineDescriptor
  {
    label : Some( "Render Pipeline") ,
    layout : None,
    vertex : wgpu::VertexState
    {
      module : &shader,
      entry_point : "vs_main",
      buffers : &[],
    },
    fragment : Some( wgpu::FragmentState
    {
      module : &shader,
      entry_point : "fs_main",
      targets : &[ wgpu::ColorTargetState
      {
        format : surface_config.format,
        blend : Some( wgpu::BlendState::REPLACE ),
        write_mask : wgpu::ColorWrites::ALL,
      }],
    }),
    primitive : wgpu::PrimitiveState
    {
      .. std::default::Default::default()
    },
    depth_stencil : None,
    multisample : wgpu::MultisampleState
    {
      .. std::default::Default::default()
    },
    multiview : None,
  });

  render_pipeline
}

//

fn context_make()
  ->
(
  winit::event_loop::EventLoop< () >,
  winit::window::Window,
  wgpu::Surface,
  wgpu::SurfaceConfiguration,
  wgpu::Device,
  wgpu::Queue,
)
{

  let event_loop = winit::event_loop::EventLoop::new();
  let window = winit::window::WindowBuilder::new().build( &event_loop ).unwrap();

  let surface_size = window.inner_size();
  let wgpu_instance = wgpu::Instance::new( wgpu::Backends::all() );

  let surface = unsafe { wgpu_instance.create_surface( &window ) };

  let adapter = pollster::block_on( wgpu_instance
  .request_adapter( &wgpu::RequestAdapterOptions
  {
    power_preference : wgpu::PowerPreference::default(),
    compatible_surface : Some( &surface ),
    force_fallback_adapter : false,
  }))
  .unwrap();

  let ( device, queue ) = pollster::block_on( adapter
  .request_device
  (
    &wgpu::DeviceDescriptor
    {
      label : None,
      features : wgpu::Features::empty(),
      limits : wgpu::Limits::default(),
    },
    None,
  )).unwrap();

  let surface_config = wgpu::SurfaceConfiguration
  {
    usage : wgpu::TextureUsages::RENDER_ATTACHMENT,
    format : surface.get_preferred_format( &adapter ).unwrap(),
    width : surface_size.width,
    height : surface_size.height,
    present_mode : wgpu::PresentMode::Fifo,
  };
  surface.configure( &device, &surface_config );

  ( event_loop, window, surface, surface_config, device, queue )
}

//

fn render< 'a >
(
  device : &wgpu::Device,
  surface : &wgpu::Surface,
  _queue : &wgpu::Queue,
  _render_pipeline : &wgpu::RenderPipeline
)
->
Result< (), wgpu::SurfaceError >
{

  let surface_texture = surface.get_current_texture()?;
  let view1 = surface_texture.texture.create_view( &wgpu::TextureViewDescriptor::default() );

  // !
  // ! #surface_texture -> #_view1
  // ! _view1 => #_color_attachments
  // ! _color_attachments => #render_pass_descriptor
  // ! #command_encoder
  // !
  // ! proper order of deconstructing:
  // ! - render_pass_descriptor
  // ! - _color_attachments
  // ! - _view1
  // ! - surface_texture
  // ! - command_encoder

  let resolve_target = None;
  let view = &view1;
  let _color_attachments : [ wgpu::RenderPassColorAttachment ; 1 ] = [ wgpu::RenderPassColorAttachment
  {
    view,
    resolve_target,
    ops : wgpu::Operations
    {
      load : wgpu::LoadOp::Clear( wgpu::Color
      {
        r : 0.8,
        g : 0.8,
        b : 0.8,
        a : 1.0,
      }),
      store : true,
    },
  }];

  let color_attachments = _color_attachments.as_ref();

  let render_pass_descriptor = wgpu::RenderPassDescriptor
  {
    label : Some( "Render Pass" ),
    color_attachments,
    depth_stencil_attachment : None,
  };

  let mut command_encoder = device.create_command_encoder( &wgpu::CommandEncoderDescriptor
  {
    label : Some( "Render Encoder" ),
  });

  let mut render_pass = command_encoder.begin_render_pass( &render_pass_descriptor );

  render_pass.set_pipeline( &_render_pipeline );
  render_pass.draw( 0..3, 0..1 );

  drop( render_pass );

  // println!( "{:?}", render_pass_descriptor );
  // println!( "{:?}", color_attachments );

  drop( surface_texture );
  drop( command_encoder );

// thread 'main' panicked at 'Texture[0] does not exist', /home/kos/.cargo/registry/src/github.com-1ecc6299db9ec823/wgpu-core-0.12.2/src/hub.rs:116:32
// stack backtrace:
//    0: rust_begin_unwind
//              at /rustc/efec545293b9263be9edfb283a7aa66350b3acbf/library/std/src/panicking.rs:498:5
//    1: core::panicking::panic_fmt
//              at /rustc/efec545293b9263be9edfb283a7aa66350b3acbf/library/core/src/panicking.rs:107:14
//    2: wgpu_core::hub::Storage<T,I>::get
//              at /home/kos/.cargo/registry/src/github.com-1ecc6299db9ec823/wgpu-core-0.12.2/src/hub.rs:116:32
//    3: <wgpu_core::hub::Storage<T,I> as core::ops::index::Index<wgpu_core::id::Valid<I>>>::index
//              at /home/kos/.cargo/registry/src/github.com-1ecc6299db9ec823/wgpu-core-0.12.2/src/hub.rs:89:9
//    4: wgpu_core::device::Device<A>::untrack
//              at /home/kos/.cargo/registry/src/github.com-1ecc6299db9ec823/wgpu-core-0.12.2/src/device/mod.rs:486:20
//    5: wgpu_core::device::<impl wgpu_core::hub::Global<G>>::command_encoder_drop
//              at /home/kos/.cargo/registry/src/github.com-1ecc6299db9ec823/wgpu-core-0.12.2/src/device/mod.rs:4192:13
//    6: <wgpu::backend::direct::Context as wgpu::Context>::command_encoder_drop
//              at /home/kos/.cargo/registry/src/github.com-1ecc6299db9ec823/wgpu-0.12.0/src/backend/direct.rs:1751:13
//    7: <wgpu::CommandEncoder as core::ops::drop::Drop>::drop
//              at /home/kos/.cargo/registry/src/github.com-1ecc6299db9ec823/wgpu-0.12.0/src/lib.rs:920:17
//    8: core::ptr::drop_in_place<wgpu::CommandEncoder>
//              at /rustc/efec545293b9263be9edfb283a7aa66350b3acbf/library/core/src/ptr/mod.rs:188:1
//    9: core::mem::drop
//              at /rustc/efec545293b9263be9edfb283a7aa66350b3acbf/library/core/src/mem/mod.rs:909:24
//   10: wgpu_texture_error::render
//              at ./src/main.rs:193:3
//   11: wgpu_texture_error::main
//              at ./src/main.rs:7:3
//   12: core::ops::function::FnOnce::call_once
//              at /rustc/efec545293b9263be9edfb283a7aa66350b3acbf/library/core/src/ops/function.rs:227:5


  // _queue.submit( std::iter::once( command_encoder.finish() ) );
  // surface_texture.present();

  Ok( () )
}

//

const SHADER : &str =
r#"

// structures

struct VertexInput
{
  [[ builtin( vertex_index ) ]] vertex_index : u32;
};

struct VertexOutput
{
  [[ builtin( position ) ]] clip_position : vec4< f32 >;
};

struct FragmentOutput
{
  [[ location( 0 ) ]] color : vec4< f32 >;
};

// Vertex shader

[[ stage( vertex ) ]]
fn vs_main
(
  input : VertexInput,
)
-> VertexOutput
{
  var out : VertexOutput;
  var x = f32( i32( input.vertex_index ) - 1 ) * 0.5;
  var y = f32( i32( input.vertex_index & 1u ) * 2 - 1 ) * 0.5;
  out.clip_position = vec4<f32>( x, y, 0.0, 1.0 );
  return out;
}

// Fragment shader

[[ stage( fragment ) ]]
fn fs_main( in : VertexOutput ) -> FragmentOutput
{
  var out : FragmentOutput;
  out.color = vec4<f32>( 0.3, 0.2, 0.1, 1.0 );
  return out;
}
"#;
