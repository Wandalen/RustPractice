#![ allow( dead_code ) ]
#![ allow( unused_variables ) ]
#![ feature( type_name_of_val ) ]

fn main()
{

}

//

trait IntoRoutine2
{
  fn into_routine_2< Ro >( src : Ro ) -> Routine< Ro >;
}

type F0< R > = fn() -> R;

impl< R > IntoRoutine2
for fn() -> R
{
  fn into_routine_2< F0 >( src : F0 ) -> Routine< F0 >
  {
    Routine { e : src }
  }
}

struct Routine< Ro >
{
  pub e : Ro
}
