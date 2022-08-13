release:
	(cargo build --release; cd target/release; tar -czf flagup-mac.tar.gz flagup; shasum -a 256 flagup-mac.tar.gz > sha256.txt)