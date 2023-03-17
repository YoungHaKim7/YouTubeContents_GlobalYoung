trait Growler {
    fn growl(&self);
}

struct Lion;
impl Growler for Lion {
    #[inline(never)]
    fn growl(&self) {
        println!("Lion says GROWL!");
    }
}
struct Tiger;
impl Growler for Tiger {
    #[inline(never)]
    fn growl(&self) {
        println!("Tiger says GROWL!");
    }
}
struct Bear;
impl Growler for Bear {
    #[inline(never)]
    fn growl(&self) {
        println!("Bear says GROWL!");
    }
}
fn static_dispatch<T: Growler>(t: T) {
    t.growl();
}

pub fn main() {
    static_dispatch(Lion {});
    static_dispatch(Tiger {});
    static_dispatch(Bear {});
}
