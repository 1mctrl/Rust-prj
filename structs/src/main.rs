#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i64,
}

fn main ()  {
    let user1 = User {
        active: true,
        username: String::from("icon"),
        email: String::from("some@mail.com"),
        sign_in_count: 1,
    };
    println!("info, {:?}", user1);
}
    
        
 
