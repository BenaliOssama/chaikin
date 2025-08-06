fn main(){
    my_function().await;
    println!("hello from main");
}


async fn my_function(){
    println!("hello from my function");
}
