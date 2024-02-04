use std::fs::read;

fn main() {
    let file = match read("roms/PONG") {
        Ok(file) => file,
        Err(_) =>{ 
            println!("Couldn't read file");
            return 
        }
    };


    
    print!("{:?}", file)
}
