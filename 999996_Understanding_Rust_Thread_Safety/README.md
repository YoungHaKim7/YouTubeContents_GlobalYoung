# Understanding Rust Thread Safety

https://onesignal.com/blog/thread-safety-rust/

# RwLock vs Mutex

The issue with this code is that the RwLock is being used to protect the Sender of the channel. However, RwLock::read returns a RwLockReadGuard, which only allows shared access to the data. Since send requires mutable access to the Sender, this code will not compile.

Here’s a fixed version of the code that uses a Mutex instead of an RwLock:

```
use std::sync::{mpsc::channel, Arc, Mutex};

pub fn main() {
    let (tx, rx) = channel();

    let x = Arc::new(Mutex::new(tx));

    std::thread::spawn(move || {
        x.lock().unwrap().send(4u8).unwrap();
    });

    dbg!(rx.recv().unwrap());

}
  
```