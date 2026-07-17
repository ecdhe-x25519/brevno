# BREVNO

A high-performance, thread-safe, `no_std` asynchronous logging library for Rust utilizing a lock-free Single-Producer Single-Consumer (SPSC) / Multi-Producer Single-Consumer (MPSC) architecture.

This library is designed for low-latency systems and embedded applications where runtime string parsing and global kernel synchronization overhead must be avoided.

## Key Features

* **Execution Speed:** Up to 100 times faster than the standard `log` facade in microbenchmarks using `std::hint::black_box`.
* **Zero Allocations:** Operates entirely without heap allocations (`no_std`). Memory for buffers is statically allocated at compile time.
* **Lock-free Pipeline:** Worker threads push structured log events into a ring buffer using atomic operations, bypassing immediate physical I/O (UART, SSD, Network).
* **Compile-time Configuration:** The total number of ring buffer slots (`NB`) and the maximum message array length (`NL`) are fixed at compile time via generics.

## Performance Evaluation

Tested with 100,000 logging operations in release mode (`--release`):

```text
=== BENCHMARK ===
BREVNO: 203.239µs
LOG:    14.89848ms
BREVNO IS 73 TIMES FASTER!
=================
```

## Internal Architecture

Instead of dynamic dispatch (`&dyn Display`) and sequential string parsing at runtime, **BREVNO** uses a lightweight adapter implementation around `core::fmt::write`. 

The LLVM optimizer inlines and unrolls the formatting logic into flat byte copying operations directly into the memory region allocated for the target thread's slot, mitigating thread contention on the hot path.

## Usage

### 1. Usage
Generate the global logger instance with chosen generic constraints at the root of your application using the initialization macro:

```rust
// Configuration: (Buffer capacity, Maximum message length in bytes, Default LogLevel)
crate::init_global_logger!(1024, 64, LogLevel::Info);
```

### 2. Emitting Logs
Invoke the logging macros from any context. The macros reference the configured static instance, evaluate the active filter level, and write bytes atomically:

```rust
use crate::info;

fn main() {
    info!("System initialized successfully");

    std::thread::spawn(|| {
        let user_id = 42;
        // Supports standard formatting traits via core::fmt
        info!("User {} established connection", user_id);
    });
}
```

### 3. Consumption
While worker threads enqueue logs asynchronously, a background consumer task processes individual entries from the buffer and writes them to the final output destination:

```rust
// Within a background processing loop
while let Some(log) = GLOBAL_LOGGER.read_log() {
    if let Ok(text) = log.decode() {
        println!("{}", text); // Outputs: [INFO] System initialized...
    }
}
```

## Technical Details

* **Atomic Filtering:** Log levels are checked using `AtomicU8` operations with `Ordering::Relaxed` sequencing, enabling minimal processing overhead for disabled levels.
* **Static Resolution:** The structure eliminates dynamic allocations during execution by ensuring that buffer sizes are strictly bounded via generic constraints.

## License

Licensed under the MIT License.
