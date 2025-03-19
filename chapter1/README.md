## The machine
Touching on 2 models of parallelism:
1. Concurrent memory operations: multiple CPUs contend to manipulate a shared, addressable memory.
2. Data parallelism: CPU is able to operate with a single or multiple instructions on multiple words
at a time concurrently.
Empirical methods such as:
- measruement
- examination of assembly code
- experimentaion with alternative implementations
will be combined with the abstract model of the machines, focusing on:
- total cache layers
- cache sizes
- bus speeds
- microcode versions

## Memory
PRAM -> Abstract machine, similar to RAM but designed for parallel computing
Types of concrete RAM:
- DRAM: cheap to produce, uses a capacitator and a transistor per cell. The electric charge of the
capacitator (actually holding the data 0 or 1) gradually leaks and without intervention, the data
eventually is lost. To prevent this DRAM requires an external memory refresh circuit which
periodically rewrites the data in the capacitators restoring them to the original charge.
- SRAM: Does not require refresh. However, it uses 4 transistors per cell, making it expensive.
it is used where speed maters: Cache hierarchies, TLBs, CAMs.
- SDRAM: Additionally to DRAM, this has its operations coordinated by an externally supplied clock
signal. Operations are coordinated based on the given set of commands. Command can be pipelined to
improve performance, with previously started operations completing while new commands are received.
Pipelining means that the chip can accept new commands before it finished the others. For a
pipelined write, the write command can be immediately followed by another command (say read at
a different address) without waiting for the data to be written into the memory array.
For a pipelined read, the requested data appears a fixed number of clock cycles (latency) after
the read command, during whichi additional commands(say write) can be sent.
Additionally to pipelining commands, the memory is split into banks, allowing the devide to operate
on a memory aceess command in each bank simultaneously and speed up access.
- DDR: Double data rate is a computer bus that transfers data on both rising and failing edges of
the clock signal and hence double the memory bandwidth by transferring data twice per clock cycle.
With data being transferred 64 bits at a time, DDR SDRAM gives a transfer rate (in bytes/s) of
(memory bus clock rate) × 2 (for dual rate) × 64 (number of bits transferred) / 8 (number of bits/byte).
Thus, with a bus frequency of 100 MHz, DDR SDRAM gives a maximum transfer rate of 1600 MB/s.

## CPU
The CPU is the device:
- decodes a stream of instruction
- executes / interprets each instruction in that stream
- manipulates storage and other devices connected to it in the process.
