use std::sync::Mutex;

fn main()
{

  let mutex1 = Mutex::new( 5 );
  println!( "mutex1 : {:?}", mutex1 );
  // < mutex1 : Mutex { data: 5, poisoned: false, .. }

  let mut mutex_gaurd = mutex1.lock().unwrap();
  println!( "mutex1 : {:?}", mutex1 );
  println!( "mutex_gaurd : {:?}", mutex_gaurd );
  // < mutex1 : Mutex { data: <locked>, poisoned: false, .. }
  // < mutex_gaurd : 5

  *mutex_gaurd += 8;
  println!( "mutex1 : {:?}", mutex1 );
  println!( "mutex_gaurd : {:?}", mutex_gaurd );
  // < mutex1 : Mutex { data: <locked>, poisoned: false, .. }
  // < mutex_gaurd : 13

  // let mut mutex_gaurd = mutex1.lock().unwrap();
  // ! would hang

  let mutex_gaurd2 = mutex1.try_lock();
  println!( "mutex_gaurd2 : {:?}", mutex_gaurd2 );
  // < mutex_gaurd2 : Err("WouldBlock")

  std::mem::drop( mutex_gaurd );
  println!( "mutex1 : {:?}", mutex1 );
  // < mutex1 : Mutex { data: 13, poisoned: false, .. }

  *mutex1.lock().unwrap() += 1;
  *mutex1.lock().unwrap() += 10;
  println!( "mutex1 : {:?}", mutex1 );
  // < mutex1 : Mutex { data: 24, poisoned: false, .. }

}