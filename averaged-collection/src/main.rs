pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    let mut avg_coll = AveragedCollection::new();
    avg_coll.add(15);
    println!("Average after adding {}: {}", 15, avg_coll.average());
    avg_coll.add(37);
    println!("Average after adding {}: {}", 37, avg_coll.average());
    avg_coll.add(4);
    println!("Average after adding {}: {}", 4, avg_coll.average());
    avg_coll.remove();
    println!("Average after removing last: {}", avg_coll.average());
}
