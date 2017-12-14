use std::collections::HashMap;
use std::sync::Arc;
use std::thread::spawn;

fn main() {
    let mut map = HashMap::new();
    map.insert("tomato", "red");
    map.insert("celery", "green");
    map.insert("carrot", "orange");

    let arc1 = Arc::new(map);
    let arc2 = arc1.clone();

    spawn(proc() println!("Celery is `{}`", arc1["celery"]));
    spawn(proc() println!("Carrots are `{}`", arc1["carrot"]));
}
