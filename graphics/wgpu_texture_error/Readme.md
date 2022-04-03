**Description**

Unexpected exception on dropping of CommandEncoder if SurfaceTexture is dropped first.

**Repro steps**

Drop `surface_texture` before dropping `command_encoder`. What confuses me `command_encoder` does not have obvious relation to `surface_texture`.

Source code which you can just run is [here](https://github.com/Wandalen/RustPractice/tree/master/graphics/wgpu_texture_error).
Regular function `render` ends with
```
  queue.submit( std::iter::once( command_encoder.finish() ) );
  surface_texture.present();
```
But if instead of that you will deconstruct ( drop ) `surface_texture` then deconstructing of `command_encoder` will throw an exception `Texture[0] does not exist`

```
  drop( surface_texture );
  // drop( command_encoder );
```

That might be bug, might be expected behavior. Anyway such behavior is very counter-intuitive for me because I don't understand relations and logic behind that. Please explain if you do.

**Expected vs observed behavior**

No exception. Order of deconstruction of 2 unrelated entities should not cause any problem.

**Extra materials**

Error:
```
// thread 'main' panicked at 'Texture[0] does not exist', /.cargo/registry/src/github.com-1ecc6299db9ec823/wgpu-core-0.12.2/src/hub.rs:116:32
// stack backtrace:
//    0: rust_begin_unwind
//              at /rustc/efec545293b9263be9edfb283a7aa66350b3acbf/library/std/src/panicking.rs:498:5
//    1: core::panicking::panic_fmt
//              at /rustc/efec545293b9263be9edfb283a7aa66350b3acbf/library/core/src/panicking.rs:107:14
//    2: wgpu_core::hub::Storage<T,I>::get
//              at /.cargo/registry/src/github.com-1ecc6299db9ec823/wgpu-core-0.12.2/src/hub.rs:116:32
//    3: <wgpu_core::hub::Storage<T,I> as core::ops::index::Index<wgpu_core::id::Valid<I>>>::index
//              at /.cargo/registry/src/github.com-1ecc6299db9ec823/wgpu-core-0.12.2/src/hub.rs:89:9
//    4: wgpu_core::device::Device<A>::untrack
//              at /.cargo/registry/src/github.com-1ecc6299db9ec823/wgpu-core-0.12.2/src/device/mod.rs:486:20
//    5: wgpu_core::device::<impl wgpu_core::hub::Global<G>>::command_encoder_drop
//              at /.cargo/registry/src/github.com-1ecc6299db9ec823/wgpu-core-0.12.2/src/device/mod.rs:4192:13
//    6: <wgpu::backend::direct::Context as wgpu::Context>::command_encoder_drop
//              at /.cargo/registry/src/github.com-1ecc6299db9ec823/wgpu-0.12.0/src/backend/direct.rs:1751:13
//    7: <wgpu::CommandEncoder as core::ops::drop::Drop>::drop
//              at /.cargo/registry/src/github.com-1ecc6299db9ec823/wgpu-0.12.0/src/lib.rs:920:17
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
```

**Platform**

Linux, wgpu 0.12.0
