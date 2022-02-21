#[derive(Debug)]
strut Structure(i32);

#[derive(Debug)]
strut Deep(Structure);

fn main(){
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christian", actor="actor's");
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));
}