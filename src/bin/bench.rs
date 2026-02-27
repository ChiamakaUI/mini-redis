use std::time::{Duration, Instant};

use rand::{Rng, rngs::StdRng, SeedableRng};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

const CLIENTS: usize = 50;
const OPS_PER_CLIENT: usize = 10_000;
const KEYSPACE: usize = 10_000;

#[tokio::main]
async fn main() {
    println!("Starting benchmark...");
    println!("Clients: {}", CLIENTS);
    println!("Ops per client: {}", OPS_PER_CLIENT);

    // Preload phase
    preload().await;

    // let latencies = Arc::new(Mutex::new(Vec::new()));
    let start = Instant::now();
    let mut handles = vec![];

    for _ in 0..CLIENTS {
        handles.push(tokio::spawn(async move {
            let stream = TcpStream::connect("127.0.0.1:8080")
                .await
                .expect("connection failed");
            let (raw_reader, mut writer) = tokio::io::split(stream);
            let mut reader = BufReader::new(raw_reader);
            let mut local_latencies = Vec::with_capacity(OPS_PER_CLIENT);
            let mut rng = StdRng::from_entropy();

            let mut response = String::new();
            for _ in 0..OPS_PER_CLIENT {
                let key_id = rng.gen_range(0..KEYSPACE);
                let cmd = if rng.gen_bool(0.8) {
                    format!("GET key{}\n", key_id)
                } else {
                    format!("SET key{} value{}\n", key_id, key_id)
                };

                let op_start = Instant::now();
                writer.write_all(cmd.as_bytes()).await.unwrap();
                response.clear();
                reader.read_line(&mut response).await.unwrap();

                local_latencies.push(op_start.elapsed());
            }
            local_latencies
        }))
    }

    let mut all_latencies = Vec::with_capacity(CLIENTS * OPS_PER_CLIENT);
    for h in handles {
        let mut results = h.await.unwrap();
        all_latencies.append(&mut results);
    }

    let duration = start.elapsed();
    let total_ops = CLIENTS * OPS_PER_CLIENT;
    let ops_per_sec = total_ops as f64 / duration.as_secs_f64();

    println!("\n=== RESULTS ===");
    println!("Total ops: {}", total_ops);
    println!("Duration: {:?}", duration);
    println!("Ops/sec: {:.2}", ops_per_sec);

    analyze_latencies(&all_latencies);
}

async fn preload() {
    let mut stream = TcpStream::connect("127.0.0.1:8080")
        .await
        .expect("preload connection failed");

    for i in 0..KEYSPACE {
        let cmd = format!("SET key{} value{}\n", i, i);
        stream.write_all(cmd.as_bytes()).await.unwrap();
    }

    println!("Preloaded {} keys", KEYSPACE);
}

fn analyze_latencies(latencies: &[Duration]) {
    if latencies.is_empty() {
        println!("No latencies recorded. Did the clients connect successfully?");
        return;
    }
    let mut sorted = latencies.to_vec();
    sorted.sort();

    let count = sorted.len();
    let avg = sorted.iter().map(|d| d.as_secs_f64()).sum::<f64>() / count as f64;
    let p95 = sorted[(count as f64 * 0.95) as usize];
    let p99 = sorted[(count as f64 * 0.99) as usize];
    let max = sorted[count - 1];

    println!("Avg latency: {:.6} ms", avg * 1000.0);
    println!("P95 latency: {:.6} ms", p95.as_secs_f64() * 1000.0);
    println!("P99 latency: {:.6} ms", p99.as_secs_f64() * 1000.0);
    println!("Max latency: {:.6} ms", max.as_secs_f64() * 1000.0);
}