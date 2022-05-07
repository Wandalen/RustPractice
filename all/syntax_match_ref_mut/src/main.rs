#![feature(type_name_of_val)]

fn main()
{

  let mut optional_val = Some( 13_i8 );
  wtools::typing::inspect_type_of!( optional_val );

  // ordinary pattern
  if let Some( val ) = optional_val
  {
    wtools::typing::inspect_type_of!( val );
  }

  // getting reference
  if let Some( ref val ) = optional_val
  {
    wtools::typing::inspect_type_of!( val );
  }

  // getting mutable reference
  if let Some( ref mut val ) = optional_val
  {
    wtools::typing::inspect_type_of!( val );
  }

  // getting reference
  if let Some( val ) = &optional_val
  {
    wtools::typing::inspect_type_of!( val );
  }

  // getting mutable reference
  if let Some( val ) = &mut optional_val
  {
    wtools::typing::inspect_type_of!( val );
  }

  /* - */

  let mut optional_ref = Some( &13_i8 );
  wtools::typing::inspect_type_of!( optional_ref );

  // ordinary pattern
  if let Some( val ) = optional_ref
  {
    wtools::typing::inspect_type_of!( val );
  }

  // getting reference
  if let Some( ref val ) = optional_ref
  {
    wtools::typing::inspect_type_of!( val );
  }

  // getting mutable reference
  if let Some( ref mut val ) = optional_ref
  {
    wtools::typing::inspect_type_of!( val );
  }

  // getting reference
  if let Some( val ) = &optional_ref
  {
    wtools::typing::inspect_type_of!( val );
  }

  // getting mutable reference
  if let Some( val ) = &mut optional_ref
  {
    wtools::typing::inspect_type_of!( val );
  }

}
