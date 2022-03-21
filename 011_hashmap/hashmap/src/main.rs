use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

fn main() {
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "123-456-7890");
    contacts.insert("Ashley", "123-456-7890");
    contacts.insert("Katie", "123-456-7890");
    contacts.insert("Robert", "123-456-7890");

    let temp = contacts.get(&"Daniel");
    assert_eq!(temp, Some(&"123-456-7890")); // 注意: &, 不是Some("123-456-7890")

    let temp2 = contacts.get(&"Daniel");
    assert_eq!(temp2, Some(&"123-456-7890")); // 注意: &, 不是Some("123-456-7890")

    // entry的常用方法
    // Entry的or_insert()方法
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("poneyland").or_insert(3);
    assert_eq!(map["poneyland"], 3);

    map.entry("poneyland").or_insert(10);
    assert_eq!(map["poneyland"], 3);
    // 如果为空，则通过插入默认值来确保该值在条目中，并返回对条目中值的可变引用。
    *map.entry("poneyland").or_insert(10) *= 2;
    assert_eq!(map["poneyland"], 6);

    // Entry的or_insert_with()方法
    let mut map: HashMap<&str, String> = HashMap::new();
    let s = "hoho".to_string();
    // 如果为空，则通过插入default闭包返回值来确保该值在条目中，并返回对条目中值的可变引用。
    map.entry("poneyland").or_insert_with(|| s);
    assert_eq!(map["poneyland"], "hoho".to_string());

    // 为了有代表性，设计了一个相对复杂的双层HashMap+内置Vec的结构，如果还不够有代表性，那就没办法了，具体如下：
    let mut hd: HashMap<&str, HashMap<&str, Box<Vec<f32>>>> = HashMap::new();
    let mut tm1 = HashMap::new();
    // 方法一, 如果tm1没有book1这个key, 则insert key 为 book1和相应的元素值
    tm1.entry("book1").or_insert(Box::new(vec![1.0_f32]));
    hd.insert("rust", tm1);
    println!("insert rust hd=>{:?}", hd);

    // 方法二,直接insert
    let mut tm2 = HashMap::new();
    tm2.insert("book2", Box::new(vec![2.0_f32]));
    hd.insert("julia", tm2);
    println!("insert julia hd=>{:?}", hd);

    // insert: Rust允许相同的key的重复操作,会进行覆盖操作.
    let mut tm3 = HashMap::new();
    tm3.insert("book1", Box::new(vec![3.0_f32]));
    hd.insert("julia", tm3);
    println!("insert julia hd=>{:?}", hd);

    // remove
    // hd.remove("julia");

    // 对vec内部元素的类似push的操作
    // hd["rust"]["book1"].push(4.0_f32); // error
    // &hd["rust"]["book1"].push(3.0_f32); // error
    // 正确应该使用get_mut()方法
    // 对于HashMap<&str, HashMap<&str, Box<Vec<f32>>>> 类型, 要对最里面进行增加元素操作:
    hd.get_mut("rust")
        .unwrap()
        .get_mut("book1")
        .unwrap()
        .push(4.0_f32);
    println!("add value => hd:{:?}", hd); // 可以看到,Vec元素增加了

    // 比如, 对于HashMap<&str, Vec<f32>>类型,要对最里面的Vec<f32>进行增加元素操作,
    let mut map: HashMap<&str, Vec<f32>> = HashMap::new();
    map.insert("rust", vec![1.0_f32]);
    map.get_mut("rust").unwrap().push(9.0_f32);
    println!("map:{:?}", map); // 可以看到,Vec元素增加了

    // key, value值
    let keys = hd.keys();
    println!("keys:{:?}", keys); // ["rust", "julia"]
    let values = hd.values();
    println!("values: {:?}", values);
    // 注意有hd.values_mut(), 但是没有keys_mut()
    // 某个key对应的值
    let value_key = hd.get("rust").unwrap().clone();
    println!("value_key: {:?}", value_key);
    // 或者注意下面&
    let value_key = &hd["rust"]["book1"];
    println!("value_key: {:?}", value_key);

    // 判断某个key是否存在
    let bool_key = hd.contains_key("rust");
    println!("bool_key: {:?}", bool_key);

    // loop
    for key in hd.keys() {
        println!("key: {:?}", key);
    }

    for (key, value) in &hd {
        println!("key: {:?}, value: {:?}", key, value);
    }
    for (key, value) in hd.iter() {
        println!("key: {:?}, value: {:?}", key, value);
    }

    // 或者
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    // iter_mut()
    for (_, val) in map.iter_mut() {
        *val *= 2;
    }

    // 注意没有keys_mut
    for val in map.values_mut() {
        *val = *val + 10;
    }

    // HashMap <==> Vec

    // HashMap => Vec
    let vec: Vec<(_, _)> = hd.into_iter().collect();
    println!("vec: {:?}", vec); // vec: [("julia", {"book1": [3.0]}), ("rust", {"book1": [1.0, 4.0]})]

    // Vec=>HashMap
    // Vec(&str, i32)
    let timber_resources: HashMap<&str, i32> = [("Norway", 100), ("Denmark", 50), ("Iceland", 10)]
        .iter()
        .cloned()
        .collect();
    println!("timber_resources: {:?}", timber_resources); // timber_resources: {"Iceland": 10, "Denmark": 50, "Norway": 100}

    // Vec
    let data = vec![1, 2, 3, 4, 5, 6];
    let map: HashMap<i64, i64> = data.iter().map(|&x| (x, x * 10)).collect();
    println!("map: {:?}", map); // map: {3: 30, 6: 60, 5: 50, 2: 20, 4: 40, 1: 10}

    // 双Vec=>HashMap
    let names = vec!["WaySLOG", "Mike", "Elton"];
    let scores = vec![60, 80, 100];
    let scroe_map: HashMap<_, _> = names.iter().zip(scores.iter()).collect();
    println!("scroe_map: {:?}", scroe_map); // scroe_map: {"WaySLOG": 60, "Mike": 80, "Elton": 100}

    // 比如，我们有两个&str，如何变成HashMap
    let c = Cipher::new(&"abc", &"123");
    println!("cipher: {:?}", c.hp); // cipher: {"b": "2", "a": "1", "c": "3"}

    // HashMap => range
    // 1. range=>HashMap
    let mut map: HashMap<i64, i64> = (0..8).map(|x| (x, x * 10)).collect();
    // 仅保留谓词指定的元素。换句话说，删除所有对 (k, v)，以使 f(&k, &mut v) 返回 false。
    map.retain(|&k, _| k % 2 == 0);
    assert_eq!(map.len(), 4);

    let map: HashMap<char, i32> = "abcdefghijklmnopqrstuvwxyz"
        .chars() // 返回字符串切片的 char 上的迭代器。
        .enumerate() // 创建一个迭代器，该迭代器给出当前迭代次数以及下一个值。返回的迭代器产生对 (i, val)，其中 i 是当前迭代索引，val 是迭代器返回的值。
        .map(|(x, y)| (y, x as i32 + 1))
        .collect();
    // map:{'u': 21, 'x': 24, 'n': 14, 'w': 23, 'c': 3, 'y': 25, 'e': 5, 'i': 9, 'p': 16, 't': 20, 'z': 26, 'r': 18, 'q': 17, 'l': 12, 'd': 4, 'j': 10, 'b': 2, 'h': 8, 'k': 11, 'm': 13, 'o': 15, 'v': 22, 'g': 7, 'f': 6, 's': 19, 'a': 1}
    println!("map:{:?}", map);
    // 2. HashMap=>Vec
    let mut v: Vec<i64> = map.iter().map(|(_, &val)| val as i64).collect();
    v.retain(|&x| x % 2 == 0);
    println!("v:{:?}", v); // v:[14, 12, 22, 4, 16, 6, 2, 8, 24, 10, 26, 18, 20]

    // keys
    let mut contacts = HashMap::new();
    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");
    // 注意，这里不能用iter(),只能用into_inter().
    let names: Vec<&str> = contacts.keys().into_iter().map(|&x| x).collect();
    let nums: Vec<&str> = contacts.values().into_iter().map(|&x| x).collect();
    println!("names: {:?}", names); // names: ["Daniel", "Ashley", "Katie", "Robert"]
    println!("nums: {:?}", nums); // nums: ["798-1364", "645-7689", "435-8291", "956-1745"]

    // 其他一些常用方法
    let mut map = HashMap::<String, String>::new();
    map.insert("foo".to_string(), "foo".to_string());
    match map.entry("foo".to_string()) {
        Vacant(entry) => {
            // 空的情况
            entry.insert("bar".to_string());
        }
        Occupied(mut entry) => {
            // 非空的情况: 取出值在后面追加内容
            entry.get_mut().push_str("bar");
        }
    }

    println!("the value of foo is => {:?}", map.get(&("foo".to_string())));

    let mut h: HashMap<&str, isize> = HashMap::new();
    h.insert("foo", 42);
    h.insert("bar", 1);
    match h.entry("foo") {
        Vacant(entry) => {
            entry.insert(1);
        }
        Occupied(mut entry) => {
            *entry.get_mut() += 1;
        }
    }
    println!("foo={:?}", h.get(&("foo")));
    assert_eq!(h["foo"], 43);

    let mut h = HashMap::<String, isize>::new();
    h.insert("foo".to_string(), 42);
    assert_eq!(h.contains_key(&"foo".to_string()), true);
    assert_eq!(h.contains_key(&"bar".to_string()), false);
    h.insert("bar".to_string(), 1);

    // List keys of the HashMap
    let mut keys: Vec<String> = Vec::new();
    for (k, _) in h.iter() {
        keys.push(k.to_string());
    }

    println!("keys: {:?}", keys); // keys: ["foo", "bar"]
    let keys = h.keys().map(|v| v.clone()).collect::<Vec<String>>();
    println!("keys: {:?}", keys); // keys: ["foo", "bar"]

    // List values fo the HashMap
    let values = h.values().map(|v| v.clone()).collect::<Vec<isize>>();
    println!("values: {:?}", values); // values: [42, 1]
}

#[derive(Debug)]
struct Cipher {
    hp: HashMap<String, String>,
}

impl Cipher {
    // 比如，我们有两个&str，如何变成HashMap
    fn new(map1: &str, map2: &str) -> Cipher {
        let map = map1
            .chars()
            .map(|x| x.to_string())
            .zip(map2.chars().map(|y| y.to_string()))
            .collect::<HashMap<String, String>>();
        Cipher { hp: map }
    }
}
