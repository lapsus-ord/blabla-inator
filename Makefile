SERVER="localhost:9092"
TOPIC="chat-topic"

list_topics:
	@echo "# List topics"
	@docker compose exec kafka kafka-topics.sh --bootstrap-server $(SERVER) --list

create_topic:
	@echo "# Creating topic"
	@docker compose exec kafka kafka-topics.sh --bootstrap-server $(SERVER) --topic $(TOPIC) --create

consume_topic:
	@echo "# Listening '$(TOPIC)' topic"
	@docker compose exec kafka kafka-console-consumer.sh --bootstrap-server $(SERVER) --topic $(TOPIC) --from-beginning

produce_topic:
	@echo "# Sending message to '$(TOPIC)' topic"
	@docker compose exec kafka kafka-console-producer.sh --bootstrap-server $(SERVER) --topic $(TOPIC)
