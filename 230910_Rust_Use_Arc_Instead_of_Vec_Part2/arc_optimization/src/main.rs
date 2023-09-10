use std::sync::Arc;

#[derive(Debug, Clone)]
struct Data1 {
    dummy_data: u32,
}

fn main() {
    let my_data1 = Data1 { dummy_data: 1 };
    let my_data2 = Data1 { dummy_data: 2 };
    let my_data3 = Data1 { dummy_data: 3 };
    let my_data4 = Data1 { dummy_data: 4 };
    let my_data5 = Data1 { dummy_data: 5 };
    let my_data6 = Data1 { dummy_data: 6 };
    let my_data7 = Data1 { dummy_data: 7 };
    let my_data8 = Data1 { dummy_data: 8 };
    let my_data9 = Data1 { dummy_data: 9 };
    let my_data10 = Data1 { dummy_data: 10 };
    let my_data11 = Data1 { dummy_data: 99 };

    let my_data_final = Data1 { dummy_data: 99 };

    let mut db = Vec::new();

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

    println!("db : {:#?}", &db);
    let arc_vec: Arc<[_]> = Arc::from(db); // 16 bytes
    println!("arc_vec_db : {:#?}", &arc_vec);
    let mut arc_vec: Vec<_> = arc_vec.to_vec().clone(); // 24bytes
    arc_vec.push(my_data11); // push pop vec로 전환
    println!("arc vec {:?} ", &arc_vec);
    let arc_vec02: Arc<[_]> = Arc::from(arc_vec); //  16bytes
}
