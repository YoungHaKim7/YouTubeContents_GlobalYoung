fn i_need_ownership(data: Option<&i32>) {
    // I need an Option<i32>
    let _: Option<i32> = data.map(ToOwned::to_owned);
    let _: Option<i32> = data.cloned();
    let _: Option<i32> = data.copied();
}

fn i_need_ownership_better(data: Option<i32>) {
    // groovy
}

fn main() {
    println!("Hello, world!");
}
