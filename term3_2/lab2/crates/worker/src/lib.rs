use structopt::StructOpt;
use redis_chat::{MessageData, Chat};
use threadpool::ThreadPool;
use fake::{faker, Fake};
use anyhow::Result;
use rand::Rng;

#[structopt(
    name = "Redis chat",
    about = "A chat console application that interacts with Redis database.",
    author = "Veetaha (github.com/Veetaha)"
)]
#[derive(StructOpt, Debug)]
pub struct Params {
    #[structopt(long, default_value = "2")]
    total_workers: usize,
    #[structopt(long, default_value = "4")]
    spam_check_latency: u64,
}

pub fn run(params: Params, chat: Chat) -> Result<()> {

    let pool = ThreadPool::with_name("workers".to_owned(), num_cpus::get());


    let smap_check_latency = std::time::Duration::from_secs(params.spam_check_latency);
    let general_latency = std::time::Duration::from_secs(2);

    for mut chat in std::iter::repeat(chat).take(params.total_workers) {
        pool.execute(move || {
            let mut random = rand::thread_rng();
            loop {
                let should_send_message: f64 = random.gen();
                if should_send_message < 0.25 {
                    let msg = faker::lorem::en::Sentence(1..2).fake();
                    let sender = faker::internet::en::Username().fake();
                    let receiver = faker::internet::en::Username().fake();

                    let msg = MessageData::new(msg, sender, receiver);

                    println!("Sending message: {}", &msg);

                    chat.send_message(msg).unwrap();
                } else {
                    println!("Checkig for spam...");
                    std::thread::sleep(smap_check_latency);
                    let res = chat.check_msg_for_spam().unwrap();
                    if let Some(detected_spam) = res {
                        println!("Checked message was {}spam", if detected_spam { "" } else { "not "});
                    } else {
                        println!("Message queue is empty, continue polling...");
                    }
                }
                std::thread::sleep(general_latency);
            }
        });
    }

    pool.join();

    Ok(())
}
