#![feature(type_name_of_val)]
#![allow(non_snake_case)]

mod anyhow
{
  pub fn Ok( src : i32 ) -> i32
  {
    src
  }
}

use crate::anyhow::Ok as Ok1;
use ::anyhow::Ok as Ok2;
// ! use anyhow::Ok as Ok3;
// <! `anyhow` is ambiguous

// use anyhow::Ok;
// use crate::anyhow::Ok;
// use ::anyhow::Ok;

// use anyhow::Ok; /* import anyhow defined either in the module or externally */
// use crate::anyhow::Ok; /* import anyhow defined only in the module */
// use ::anyhow::Ok; /* import anyhow been only external module */

fn main()
{
  println!( "{:?}", Ok1( 13 ) );
  println!( "{:?}", Ok2( 13 ) );
}
