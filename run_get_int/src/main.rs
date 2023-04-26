extern "C" {
    fn get_integer() -> i32;
}

fn main() {
    let got = unsafe { get_integer() };
    println!("Got integer: {got}");
}
