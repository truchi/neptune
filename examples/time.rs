use neptune::*;
use reqwest::Client;
use std::time::{Duration, Instant};

const SYMBOL: &'static str = "EUR";
const BATCH_SIZE: usize = K4;
const LIMIT: usize = K8;

#[derive(Default, Debug)]
struct Data {
    first: Vec<(usize, [Duration; 9])>,
    last: Vec<(usize, [Duration; 9])>,
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    let port = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("3000"));
    let mut data = Data::default();

    println!("BATCH_SIZE: {BATCH_SIZE}, LIMIT: {LIMIT}, PORT: {port}");

    //
    // Run POST and GET request until LIMIT
    //

    let mut count = 0;

    while count < LIMIT {
        count += BATCH_SIZE;
        let mut durations = [Duration::ZERO; 9];

        let now = Instant::now();
        client
            .post(format!("http://127.0.0.1:{port}/add_batch"))
            .json(&AddBatchPayload {
                symbol: String::from(SYMBOL),
                values: std::array::from_fn::<_, BATCH_SIZE, _>(|_| rand::random::<i16>() as Float)
                    .to_vec(),
            })
            .send()
            .await
            .unwrap();
        durations[0] = now.elapsed();

        for k in KS {
            let now = Instant::now();
            client
                .get(format!("http://localhost:{port}/stats"))
                .query(&StatsQuery {
                    symbol: String::from(SYMBOL),
                    k,
                })
                .send()
                .await
                .unwrap()
                .json::<StatsResponse>()
                .await
                .unwrap();
            durations[k] = now.elapsed();
        }

        println!("{count} {durations:?}");

        if data.first.len() < 10 {
            data.first.push((count, durations));
        } else {
            data.last.push((count, durations));

            if data.last.len() > 10 {
                data.last.remove(0);
            }
        }
    }

    //
    // Print summary
    //

    fn print(data: &[(usize, [Duration; 9])]) {
        for (count, durations) in data {
            let [post, get1, get2, get3, get4, get5, get6, get7, get8] =
                durations.map(|duration| duration.as_micros());

            println!("{count:>9} | {post:>6}µs | {get1:>6}µs | {get2:>6}µs | {get3:>6}µs | {get4:>6}µs | {get5:>6}µs | {get6:>6}µs | {get7:>6}µs | {get8:>6}µs");
        }
    }

    println!("============================================================================================================");
    println!(" First 10 |     POST |   GET k1 |   GET k2 |   GET k3 |   GET k4 |   GET k5 |   GET k6 |   GET k7 |   GET k8");
    println!("------------------------------------------------------------------------------------------------------------");
    print(&data.first);
    println!("------------------------------------------------------------------------------------------------------------");
    println!("  Last 10 |     POST |   GET k1 |   GET k2 |   GET k3 |   GET k4 |   GET k5 |   GET k6 |   GET k7 |   GET k8");
    println!("------------------------------------------------------------------------------------------------------------");
    print(&data.last);
    println!("============================================================================================================");
}
