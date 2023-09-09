#[derive(Debug)]
struct Data1 {
    dummy_num: u32,
}

fn main() {
    let my_data1 = Data1 { dummy_num: 9 };
    let my_data2 = Data1 { dummy_num: 9 };
    let my_data3 = Data1 { dummy_num: 9 };
    let my_data4 = Data1 { dummy_num: 9 };
    let my_data5 = Data1 { dummy_num: 9 };
    let my_data6 = Data1 { dummy_num: 9 };
    let my_data7 = Data1 { dummy_num: 9 };
    let my_data8 = Data1 { dummy_num: 9 };
    let my_data9 = Data1 { dummy_num: 9 };
    let my_data10 = Data1 { dummy_num: 9 };

    let db = vec![
        my_data1, my_data2, my_data3, my_data4, my_data5, my_data6, my_data7, my_data8, my_data9,
        my_data10,
    ];

    dbg!(db);
}
