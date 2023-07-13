build:
	cd hello && \
	cargo build && \
 	mkdir -p output/bin/ && \
 	cp ./target/debug/hello ./output/bin/ && \
 	if [ -d dependency ]; then cp -r dependency output; fi