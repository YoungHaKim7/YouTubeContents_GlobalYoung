use std::any::Any;

trait Trait: Any {
    fn msg(&self) -> String;
}

fn pair(a: *const dyn Trait, b: *const dyn Trait) {
    let a_msg;
    let b_msg;
    unsafe {
        a_msg = a.as_ref().unwrap().msg();
        b_msg = b.as_ref().unwrap().msg();
    }

    let a_data_addr = a as *const ();
    let b_data_addr = b as *const ();

    if a_msg != b_msg && a_data_addr == b_data_addr {
        println!("Found one!!");
        dbg!(a_data_addr);
        dbg!(b_data_addr);
        dbg!(a);
        dbg!(b);
        dbg!(a_msg);
        dbg!(b_msg);
    }
}

struct Zst;
struct Msg(&'static str);

impl Trait for Zst {
    fn msg(&self) -> String {
        format!("Zst")
    }
}

impl Trait for Msg {
    fn msg(&self) -> String {
        format!("Msg({})", self.0)
    }
}

fn main() {
    let zm = (Zst, Msg("good gravy"));
    let zm: (&dyn Trait, &dyn Trait) = (&zm.0, &zm.1);
    pair(zm.0, zm.1);

    dbg!((*zm.0).type_id());
    dbg!((*zm.1).type_id());
}
