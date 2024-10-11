DB_DOCKER_CONTAINER=project-rust

# IMPORTANT! please check that in the game model in the CreateSchema
# the day attribute is set to STRING -> pub day: String -> DONT USE CHRONO FOR day!

install:
# uncomment and indent
	cargo install cargo-edit
	cargo add actix-web
	cargo add actix-cors
	cargo add serde_json
	cargo add serde --features derive
	cargo add chrono --features serde
	cargo add env_logger
	cargo add dotenv
	cargo add uuid --features "serde v4"
	cargo add sqlx --features "runtime-async-std-native-tls postgres chrono uuid"
# SQLX-CLI
	cargo install sqlx-cli