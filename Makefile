debug: build
	ya-dbg --runtime target/debug/golem-postgresql-runtime --workdir .

build: target/debug/golem-postgresql-runtime
	cargo build
