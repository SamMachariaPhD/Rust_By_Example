fn main(){
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{subject}{verb}{object}",subject = "The quick brown fox ", verb = "jump over ", object = "the lazy dog.");
    println!("{} of {:b} people know binary, the other half doesn't", 1,2);
    println!("{number:>width$}", number=1, width=6);
    println!("{number:0>width$}", number=1, width=6);
    println!("My name is {0}, {1} {0}", "Bond", "James");
    //#[allow(dead_code)]
    //strut Structure(i32);
    //println!("This strut `{}` won't print...", Structure(3));
    let pi = 3.141592;
    println!("Pi is roughly {:.3}",pi); // format to 3 decimals 
}