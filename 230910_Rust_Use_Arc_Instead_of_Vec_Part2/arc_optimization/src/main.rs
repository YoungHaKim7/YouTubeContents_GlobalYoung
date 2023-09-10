use std::sync::Arc;

#[derive(Debug)]
struct Data1 {
    dummy_num: u32, // 4bytes
}

fn main() {
    let my_data1 = Data1 { dummy_num: 1 };
    let my_data2 = Data1 { dummy_num: 2 };
    let my_data3 = Data1 { dummy_num: 3 };
    let my_data4 = Data1 { dummy_num: 4 };
    let my_data5 = Data1 { dummy_num: 5 };
    let my_data6 = Data1 { dummy_num: 6 };
    let my_data7 = Data1 { dummy_num: 7 };
    let my_data8 = Data1 { dummy_num: 8 };
    let my_data9 = Data1 { dummy_num: 9 };
    let my_data10 = Data1 { dummy_num: 10 };
    let my_data11 = Data1 { dummy_num: 11 };

    let mut db = Vec::new(); // 24bytes

    db.push(my_data1);
    db.push(my_data2);
    db.push(my_data3);
    db.push(my_data4);
    db.push(my_data5);
    db.push(my_data6);
    db.push(my_data7);
    db.push(my_data8);
    db.push(my_data9);
    db.push(my_data10);
    db.push(my_data11);
    let shared: Arc<[_]> = Arc::from(db); // 16bytes

    println!("{:#?}", &shared);
}
