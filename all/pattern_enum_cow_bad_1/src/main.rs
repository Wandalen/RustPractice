use std::borrow::Cow;

fn main()
{
  for num in 0..=5
  {
    match rem3( num )
    {
      Cow::Borrowed( msg ) => println!( "{} : borrowed : {}", num, msg ),
      Cow::Owned( msg ) => println!( "{} : owned : {}", num, msg ),
    }
  }
  // 0 : borrowed : rem3 is 0
  // 1 : borrowed : rem3 is 1
  // 2 : owned : rem3 is 2
  // 3 : borrowed : rem3 is 0
  // 4 : borrowed : rem3 is 1
  // 5 : owned : rem3 is 2
}

//

fn rem3( src : i32 ) -> Cow< 'static, str >
{
  match src % 3
  {
    0 => "rem3 is 0".into(),
    1 => "rem3 is 1".into(),
    rem => format!( "rem3 is {}", rem ).into(),
  }
}