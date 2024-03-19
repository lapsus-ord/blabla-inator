use blabla_inator::{cli_config::Cli, error::Error, kafka::Kafka, utils::clean_line};
use clap::Parser;
use rdkafka::util::get_rdkafka_version;
use tokio::io::{AsyncBufReadExt, BufReader};
use tracing::{error, info, warn};
use tracing_log::AsTrace;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    cli.init_logging();

    info!("Starting Kafka producer on {}", cli.server);
    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let producer_config = Kafka::new(&cli.server, &cli.topic);
    let consumer_config = producer_config.clone();

    let producer_thread = tokio::spawn(async move {
        produce_logic(&producer_config).await;
    });

    let consumer_thread = tokio::spawn(async move {
        consume_logic(&consumer_config).await;
    });

    tokio::select! {
        _ = producer_thread => (),
        _ = consumer_thread => (),
    }

    Ok(())
}

async fn produce_logic(config: &Kafka) {
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();

    loop {
        line.clear();

        let read_line_result = reader.read_line(&mut line).await;
        if let Err(e) = read_line_result {
            warn!("skip, error reading line: {:?}", e);
            continue;
        }

        clean_line(&mut line);
        if line.is_empty() {
            warn!("skip, because of empty line: {:?}", &line);
            continue;
        }

        let _ = config
            .produce(&line)
            .await
            .map_err(|e| error!("error producing message: {:?}", e));
    }
}

async fn consume_logic(config: &Kafka) {
    let _ = config
        .consume(|message| println!("{}", message))
        .await
        .map_err(|e| error!("error consuming message: {:?}", e));
}
