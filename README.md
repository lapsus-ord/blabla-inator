# blabla-inator

Chat app using CLI to discuss through Kafka.

## Get started

If you don't have a Kafka running, you can run a single instance with:

```bash
docker compose up -d
```

> **Note:** there is available commands to interact with Kafka in the [`Makefile`](./Makefile).
>
> And you can create a topic (here `chat-topic`) with the command `make create_topic`.

Then, you can run the app with:

```bash
cargo run -- --server <your-server> --topic <your-topic>
```

(Replace `<your-server>` and `<your-topic>` with your own values.)
