valgrind-cachegrind-special:
	valgrind --tool=cachegrind --branch-sim=yes target/release/specialized

valgrind-cachegrind-std:
	valgrind --tool=cachegrind --branch-sim=yes target/release/standard

valgrind-cachegrind-naive:
	valgrind --tool=cachegrind --branch-sim=yes target/release/naive

linux-perf-special:
	perf stat --event task-clock,context-switches,page-faults,cycles,instructions,branches,branch-misses,cache-references,cache-misses target/release/specialized > /dev/null

linux-perf-std:
	perf stat --event task-clock,context-switches,page-faults,cycles,instructions,branches,branch-misses,cache-references,cache-misses target/release/std > /dev/null
