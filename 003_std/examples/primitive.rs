#![feature(array_zip)]
#![feature(array_methods)]
#![feature(bool_to_option)]



fn main() {
    // array
    let mut array: [i32; 3] = [0; 3];
    array[1] = 1;
    array[2] = 2;

    assert_eq!([1,2], &array[1..]);
    // print
    for x in array {
        println!("{} ", x);
    }

    for item in array.iter().enumerate() {
        let (i, x): (usize, &i32) = item;
        println!("array[{}] = {}", i, x);
    }

    for item in IntoIterator::into_iter(array).enumerate() {
        let (i, x): (usize, i32) = item;
        println!("array[{}] = {}", i, x);
    }

    let x = [1, 2, 3];
    let y = x.map(|v| v + 1);
    assert_eq!(y, [2, 3, 4]);

    let x = [1,2,3];
    let mut temp = 0;
    let y = x.map(|v| {temp+=1; v*temp});
    assert_eq!(y, [1,4,9]);

    let x = ["Ferris", "Bueller's", "Day", "Off"];
    let y = x.map(|v|v.len());
    assert_eq!(y, [6,9,3,3]);

    let x = [1,2,3];
    let y = [4,5,6];
    let z = x.zip(y);
    assert_eq!(z, [(1,4), (2,5), (3,6)]);

    let floats = [3.1, 2.7, -1.0];
    let float_refs: [&f64; 3] = floats.each_ref();
    assert_eq!(float_refs, [&3.1, &2.7, &-1.0]);

    let strings = ["Ferris".to_string(), "♥".to_string(), "Rust".to_string()];
    let is_ascii = strings.each_ref().map(|s| s.is_ascii());
    assert_eq!(is_ascii, [true, false, true]);

   // 我们仍然可以访问原始数组: 它尚未移动。
    assert_eq!(strings.len(), 3);

    assert_eq!(true as i32, 1);
    assert_eq!(false as i32, 0);

    assert_eq!(false.then_some(0), None);
    assert_eq!(true.then_some(32.5), Some(32.5));

    // pointer
    let my_num: i32 = 10;
    let my_num_ptr: *const i32 = &my_num;
    let mut my_speed: i32 = 88;
    let my_speed_ptr: *mut i32 = &mut my_speed;

    let my_num: Box<i32> = Box::new(10);
    let my_num_ptr: *const i32 = &*my_num;
    let mut my_speed: Box<i32> = Box::new(88);
    let my_speed_ptr: *mut i32 = &mut *my_speed;
}