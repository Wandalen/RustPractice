
fn re( v : Vec<i32> ) -> Vec<i32>
{
    println!( "re : {:?}", v[ 0 ] );
    v
}

//

fn borrow( v : &Vec<i32> )
{
    println!( "mem_borrow_1 : {:?}", (*v)[ 0 ] );
    println!( "borrow2 : {:?}", &v[ 0 ] );
    println!( "borrow3 : {:?}", v[ 0 ] );
}

//

fn count( v : &Vec<i32>, val : i32 ) -> usize
{
    v.into_iter().filter( |&&x| x == val ).count()
}

//

fn main()
{

    let mut v = Vec::new();
    for i in 1..10
    {
        v.push( i );
    }
    v = re( v );
    println!( "after re : {:?}", v[ 0 ] );

    borrow( &v );
    println!( "after borrow : {:?}", v[ 0 ] );

    let v2 = vec![ 1, 2, 2, 3 ];
    for &i in &v2
    {
        let c = count( &v2, i );
        println!( "{:?} : {:?}", i, c );
    }

}
