fn main() {
    println!("Hello, world!");
    let mut x =5;
    println!(" {x}");
    x = 6;
    println!(" {x}");
    // 
    let y ;  // immutable vaiable but not value assigned
    if x == 6{
        y = 10;
    }
    else{
        y = 20;
    }
    println!(" {y}");
    let z: char = ':';
    println!(" {z}");
    let tup: (i32, f64,u8) = (500, 6.4, 1);
    let xyz: (&str, &str) = ("Hello", "there");
    println!("j {}",tup.0);
    println!("u  {}",xyz.1);
    assert!(tup.1==54.0); //if fails it will pacnnic (error) and stop the code
    let vote: char = 'V';
    enum Work
    {
        Civilian,
        Soldier,
    }
    struct Point{
        x: i32,
        y : i32,
    }
    impl Point {
        fn add(x: i32, y:i32) -> Point
        {
            Point { x:x+y, y:10}
        }
    }
    let ra  = Point::add(1,2,);
                    
}
