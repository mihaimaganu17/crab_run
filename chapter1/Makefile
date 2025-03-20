memcheck:
	valgrind --tool=memcheck target/x86_64-unknown-linux-gnu/debug/hello_world

# Profiling memory use
massif:
	valgrind --tool=massif target/x86_64-unknown-linux-gnu/debug/hello_world

# Profiling cache
cachegrind:
	valgrind --tool=cachegrind target/x86_64-unknown-linux-gnu/debug/hello_world
