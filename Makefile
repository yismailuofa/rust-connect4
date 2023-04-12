
data_dir:
	mkdir -p data

run: data_dir
	cd server && cargo run&
	mongod --port 27017 --dbpath ./data/&
	cd client && trunk serve --open

clean:
	rm -rf data
	cd server && cargo clean
	cd client && cargo clean