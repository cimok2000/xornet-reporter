#[derive(Debug)]
struct Reporter {
    pub data_dollector: DataCollector,
    pub version: String,
}

impl Reporter {
    fn new() -> Self {
        let data_dollector: DataCollector = DataCollector::new();
        let version: String = env!("CARGO_PKG_VERSION").to_string();

        return Self {
            data_dollector,
            version,
        };
    }
}

#[derive(Debug)]

struct DataCollector {}

impl DataCollector {
    fn new() -> Self {
        return Self {};
    }
}

fn main() {
    let reporter: Reporter = Reporter::new();
    println!("{:?}", reporter);
}
