#[derive(Debug)]
struct Bat {
    bat_name: String,
    bat_size: f32,
}

#[derive(Debug)]
struct BatCollection(Vec<Bat>);
impl BatCollection {
    fn new() -> BatCollection {
        BatCollection(Vec::new())
    }
    fn add(&mut self, elem: Bat) {
        self.0.push(elem);
    }
}

impl IntoIterator for BatCollection {
    type Item = Bat;
    type IntoIter = ::std::vec::IntoIter<Bat>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

fn main() {
    let mut collection =  BatCollection::new();
    let bat_one = Bat {
        bat_name: "Fluffy".to_string(),
        bat_size: 55.4,
    };

    let bat_two = Bat {
        bat_name: "Jarry".to_string(),
        bat_size: 99.5,
    };

    collection.add(bat_one);
    collection.add(bat_two);

    for (itter, cur) in collection.into_iter().enumerate() {
        println!("Number: {:?} and the Bat's Name was {:?}",
                 itter,
                 cur.bat_name);
    }
}
