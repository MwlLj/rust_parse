pub fn u8ArrSplit<F>(data: &[u8], c: u8, callback: &mut F) -> Result<(), std::io::Error>
    where F: FnMut(&u8, &str) {
    let mut index = 0;
    let mut s = String::new();
    for item in data.iter() {
        if *item == c {
            callback(&index, &s);
            index += 1;
            s.clear();
        } else {
            s.push(*item as char);
        }
    }
    callback(&index, &s);
    Ok(())
}

#[test]
#[ignore]
pub fn u8ArrSplitTest() -> Result<(), std::io::Error> {
    let v = [0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 124, 124, 124, 49, 57, 50, 46, 49, 54, 56, 46, 57, 46, 49, 53, 124, 49, 50, 51, 52, 53, 54];
    u8ArrSplit(&v, '|' as u8, &mut |index: &u8, field: &str| {
        if *index == 0 {
            println!("index0: {:?}", field);
        } else if *index == 1 {
            println!("index1: {:?}", field);
        } else if *index == 2 {
            println!("index2: {:?}", field);
        } else if *index == 3 {
            println!("index3: {:?}", field);
        } else if *index == 4 {
            println!("index4: {:?}", field);
        }
    });
    Ok(())
}
