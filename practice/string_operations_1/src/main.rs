
use std::any::type_name;

//

fn type_name_of<T>(_: T) -> &'static str
{
    type_name::<T>()
}

//

fn main()
{

    let str1 = String::from( "Str1 " );
    println!( "Str1 : {} of {:?}", &str1, type_name_of( &str1 ) );
    let str2 = "This is str2".to_string();
    println!( "Str2 : {} of {:?}", &str2, type_name_of( &str2 ) );

    let slice1 = "slice1";
    println!( "slice1 : {} of {:?}", &slice1, type_name_of( &slice1 ) );
    let slice2 = &str1[ 1..3 ];
    println!( "slice2 : {} of {:?}", &slice2, type_name_of( &slice2 ) );

    let a = String::from( "Hello " );
    let b = String::from( "World!" );
    let ab = a + &b;
    // println!( "a : {} of {:?}", &a, type_name_of( &a ) );
    println!( "b : {} of {:?}", &b, type_name_of( &b ) );
    println!( "ab : {} of {:?}", &ab, type_name_of( &ab ) );

    let str3 = "This is str3".to_string();
    println!( "str3 : {} of {:?}", &str3, type_name_of( &str2 ) );
    println!( "str3.len : {}", str3.len() );
    println!( "str3.is_empty : {}", str3.is_empty() );
    println!( "str3.contains : {}", str3.contains( "is" ) );
    println!( "str3.as_ptr : {:?}", str3.as_ptr() );

    for token in str3.split_whitespace()
    {
        println!( "{}", token );
    }

    for token in str3.split_ascii_whitespace()
    {
        println!( "{}", token );
    }

    for token in "this is a slice".split_ascii_whitespace()
    {
        println!( "{}", token );
    }

    let mut mutable_str = String::from( "mutable_str" );
    println!( "mutable_str : {}", &mutable_str );
    mutable_str.push_str( " - yes it is" );
    println!( "mutable_str : {}", &mutable_str );

}
