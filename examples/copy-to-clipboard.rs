use std::io;
fn main(){
    println!("Hello, world!");
    let mut stdout=io::stdout();
    let mut args=std::env::args();
    args.next();
    let default_text=String::from("Example text");
    match args.next().as_deref() {

    }



}