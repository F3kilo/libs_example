fn main() {
    unsafe {
        // load library
        let lib = libloading::Library::new("target/debug/libmy_get_int.so").unwrap();

        // load function from library
        let get_int_fn = lib.get::<fn() -> i32>(b"get_integer").unwrap();

        // call loaded function
        let got =  get_int_fn();
        println!("Got integer: {got}");
    }
}
