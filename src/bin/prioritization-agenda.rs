use triagebot::agenda;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let agenda = agenda::prioritization();

    print!("{}", agenda.call().await);
}
