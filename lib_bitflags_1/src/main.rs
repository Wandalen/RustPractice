#[macro_use]
extern crate bitflags;

//

bitflags!
{
  struct Spices : u32
  {
    const SALT = 0b_0001;
    const PEPPER = 0b_0010;
    const CHILI = 0b_0100;
    const SHAFRON = 0b_1000;
    const ALL = Self::SALT.bits | Self::PEPPER.bits | Self::CHILI.bits | Self::SHAFRON.bits;
  }
}

//

impl Spices
{
  pub fn clear( &mut self ) -> &mut Self
  {
    self.bits = 0;
    self
  }
}

//

fn main()
{

  let classic = Spices::SALT | Spices::PEPPER;
  println!( "Classic : {:?}", classic );
  println!( "Bits : {:08b}", classic.bits() );
  let spicy = Spices::CHILI | Spices::PEPPER;
  println!( "Spicy : {:?}", spicy );
  println!( "Bits : {:08b}", spicy.bits() );

  println!( "Union : {:?}", classic | spicy );
  println!( "Inter : {:?}", classic & spicy );
  println!( "Diff : {:?}", classic - spicy );
  println!( "Complement : {:?}", !classic );

  let mut custom = classic | spicy;
  println!( "Custom {:?}", custom );
  custom.insert( Spices::SHAFRON );
  println!( "Insert {:?}", custom );
  custom.toggle( Spices::SHAFRON );
  println!( "Toggle {:?}", custom );
  custom.remove( Spices::CHILI );
  println!( "Remove {:?}", custom );
  custom.set( Spices::CHILI, true );
  println!( "Set {:?}", custom );

  let bits = 0b_1101;
  if let Some( from1 ) = Spices::from_bits( bits )
  {
    println!( "From1 {:?}", from1 );
  }

}
