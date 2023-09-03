fn main() {
    let x_int = 32;
    let my_string = "test_helix_debugger";
    let box_string = Box::new("test");
    println!("int : test {x_int}, /n String test : {my_string}");
    println!("Box string = {box_string}");

    for xs in x_int..52 {
        println!("int test +1 = {xs}");
    }
}
