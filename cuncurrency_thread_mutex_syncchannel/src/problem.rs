// use std::sync::{ mpsc, Arc, Mutex };
// use std::thread;
// use std::error::Error;

// // fn main() 
// // {
// // }

// fn sh_rx_get( sh_rx : Arc< Mutex< mpsc::Receiver< usize > > > ) -> Result< usize, Box<dyn Error> >
// {
//   let rx = sh_rx.lock()?;
//   let r = rx.try_recv()?;
//   Ok( r )
// }
