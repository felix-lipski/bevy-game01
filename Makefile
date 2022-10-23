scf4: src/main.rs
	cargo run --features bevy/dynamic
	# alacritty --hold --command cargo run --features bevy/dynamic


scf5: src/main.rs
	cargo run --features bevy/dynamic
	cat ./render-graph.dot | dot -Tsvg > render-graph.svg
	sxiv render-graph.svg
