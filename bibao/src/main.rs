use std::collections::HashMap;


struct Cacher<T>
    where T: Fn(u32) -> u32 {
        map: HashMap<u32, u32>,
        calculation: T,
        // value: Option<u32>,
    }

// TODO 提高一下Cacher闭包的覆盖范围，比如可以是任意个参数，以及任意个返回的场景
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            map: HashMap::new(),
            calculation,
            // value: None,
        }
    }

    fn map(&mut self, arg: u32) -> u32 {
        match self.map.get(&arg) {
            Some(value) => *value,
            None => {
                let v = (self.calculation)(arg);
                self.map.insert(arg, v);
                v
            }
        }
    }

    // fn value(&mut self, arg: u32) -> u32 {
    //     match self.value {
    //         Some(v) => v,
    //         None => {
    //             let v = (self.calculation)(arg);
    //             eprintln!("value is:{}", v);
    //             self.map.entry(arg).or_insert(v);
    //             self.value = match self.map.get(&v) {
    //                 Some(value) => value,
    //                 None => Some(v),
    //             };
    //             eprintln!("self.value is: {:?}", self.value);
    //             v
    //         },
    //     }
    // }
}



#[test]
fn test() {
    println!("begin test!");
    let mut c = Cacher::new(|a| {a});

    let v1 = c.map(1);
    let v2 = c.map(2);


    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}



fn main() {
    println!("hello");
}
