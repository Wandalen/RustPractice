use std::borrow::Borrow;
use std::hash::Hash;
use std::marker::PhantomData;
// use std::collections::HashMapCustom;

fn main()
{
  let mut map = HashMapCustom::new();

  map.insert( "ab".to_string(), 3 );

  // map.insert( "ab", 13 );

  // {
  //   map.insert( "ab", 23 );
  // }

  {
    let key = "ab".to_string();
    map.insert( key, 33 );
  }

  if let Some( val ) = map.get( "ab" )
  {
    println!( "val : {}", val );
  }

  // let string1 : String = "ab".to_string();
  // let slice1 : &str = &string1;

  let key1 = "ab".to_string();
  if let Some( val ) = map.get( &key1 )
  {
    println!( "val : {}", val );
  }

}

pub struct HashMapCustom< KeyContent : Hash + Eq + ?Sized, KeyWrap : Borrow< KeyContent > + std::cmp::PartialEq, V >
{
  keys : Vec< KeyWrap >,
  vals : Vec< V >,
  phantom: PhantomData< KeyContent >,
}

impl< KeyContent : Hash + Eq + ?Sized, KeyWrap : Borrow< KeyContent > + std::cmp::PartialEq, V >
HashMapCustom< KeyContent, KeyWrap, V >
{

  pub fn new() -> Self
  {
    let keys = Vec::new();
    let vals = Vec::new();
    let phantom:PhantomData<KeyContent> = Default::default();
    Self { keys, vals, phantom }
  }

  pub fn insert( &mut self, key : KeyWrap, val : V )
  {
    let index_maybe = self.keys.iter().position( | e | *e == key );
    if let Some( index ) = index_maybe
    {
      self.keys[ index ] = key;
      self.vals[ index ] = val;
    }
    else
    {
      self.keys.push( key );
      self.vals.push( val );
    }
  }

  pub fn get( &self, key : &KeyContent ) -> Option<&V>
  {
    let index = self.keys.iter().position( | e | e.borrow() == key )?;
    Some( &self.vals[ index ] )
  }

}
