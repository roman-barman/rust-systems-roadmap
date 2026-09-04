# Roadmap: Systems Programmer in Rust

Path: **Foundations → Cloud/Infrastructure → OS & Virtualization → Embedded**

Logic: Stage 1 gets you your first job, Stage 2 gives you depth and access to top-tier teams, Stage 3 is an optional expansion you'll enter with a 90% head start.

---

## Stage 0. Foundations (2–4 months)

### Advanced Rust
- [ ] Lifetimes in complex cases, HRTB
- [ ] Smart pointers: `Box`, `Rc`, `Arc`, `RefCell`, `Cow`
- [ ] Unsafe Rust: raw pointers, FFI, invariants — *Rustonomicon*
- [ ] Concurrency: `Send`/`Sync`, atomics, memory ordering — *Rust Atomics and Locks* (Mara Bos, free online)
- [ ] Async in depth: how Future works, pin, executors — tokio mini-redis tutorial

### C and low-level
- [ ] C: K&R or *Modern C* (1–2 months)
- [ ] Project: your own malloc
- [ ] Project: a simple shell
- [ ] CS:APP — chapters on memory, linking, assembly
- [ ] Habit: inspect your Rust code in Godbolt (read assembly, don't write it)

### OS fundamentals
- [ ] *OSTEP* (Operating Systems: Three Easy Pieces, free): processes, threads, syscalls, memory

**Checkpoint:** I read C fluently, understand Godbolt output, and have written a multithreaded program with atomics.

---

## Stage 1. Cloud + Infrastructure (6–9 months) → first job

### Networking (2–3 months)
- [ ] TCP/UDP, sockets — *Beej's Guide to Network Programming*
- [ ] TLS, HTTP/1.1 → HTTP/2 → HTTP/3 (QUIC), DNS
- [ ] epoll / io_uring — how tokio works under the hood
- [ ] Project: HTTP server on raw sockets
- [ ] Project: reverse proxy (take inspiration from Cloudflare's Pingora)

### Linux (in parallel)
- [ ] Namespaces, cgroups — the foundation of containers
- [ ] strace, perf, eBPF basics (bpftrace)
- [ ] Project: mini container runtime in Rust

### Storage (2–3 months)
- [ ] LSM trees, B-trees, WAL, MVCC
- [ ] PingCAP Talent Plan (Rust-based course)
- [ ] Project: KV store with persistence
- [ ] Read: *DDIA* (2nd edition, ~1 chapter per 1–2 weeks)
- [ ] Read source code: redb, sled

### Distributed systems (2–3 months)
- [ ] MIT 6.824 (lectures are free)
- [ ] Raft — understand the algorithm
- [ ] **Showcase project:** KV store with Raft replication

### Tooling (along the way)
- [ ] Docker + Kubernetes (confident usage + internals)
- [ ] gRPC, Protobuf
- [ ] Prometheus, OpenTelemetry
- [ ] flamegraph, tokio-console
- [ ] One cloud provider (AWS is the most common in job postings)

### In the background
- [ ] Buy an RP2040 (~$5), blink LEDs with Embassy on weekends

**Checkpoint:** 2–3 projects on GitHub, DDIA finished, applying to jobs (junior/mid in infrastructure).

---

## Stage 2. OS & Virtualization (years 2–3, alongside work)

### Your own OS
- [ ] The *Writing an OS in Rust* blog series (Philipp Oppermann) in full
- [ ] Task scheduler (round-robin → priorities)
- [ ] Userspace: syscalls, ring 0 → ring 3
- [ ] Simple filesystem (FAT32)
- [ ] A driver (keyboard, virtio, or AHCI)
- [ ] Read xv6 (teaching OS in C)
- [ ] Reference: OSDev Wiki

### Virtualization (the bridge between work and OS)
- [ ] KVM API: a minimal hypervisor in Rust via /dev/kvm
- [ ] virtio: how the guest talks to the host
- [ ] Read source code: Firecracker, Cloud Hypervisor

### eBPF in depth
- [ ] aya (Rust framework for eBPF)

**Checkpoint:** my own mini-OS with userspace, a working toy hypervisor, I understand Firecracker's code.

---

## Stage 3. Embedded (year 3+, if the interest holds)

- [ ] Hardware: STM32 (Nucleo) or RP2040
- [ ] *The Embedded Rust Book* + *Discovery Book*
- [ ] The stack: PAC → HAL → BSP, `embedded-hal` traits
- [ ] Interrupts, timers, DMA
- [ ] Protocols with real sensors: UART, SPI, I2C
- [ ] Embassy (async) and RTIC (realtime)
- [ ] Debugging: probe-rs, defmt, GDB over SWD
- [ ] **Showcase project:** a chip driver published as a crate on crates.io, or a USB device (HID)

---

## Ongoing habits

- [ ] Read other people's code constantly: tokio → TiKV → Firecracker → Embassy
- [ ] Contribute to open source starting from Stage 1 (TiKV, Vector, Quickwit)
- [ ] Keep notes / a blog about your projects

## Key books

| Book | Stage |
|---|---|
| Rustonomicon | 0 |
| Rust Atomics and Locks | 0 |
| CS:APP | 0 |
| OSTEP | 0 |
| Beej's Guide to Network Programming | 1 |
| DDIA (2nd ed.) | 1 |
| The Embedded Rust Book + Discovery | 3 |
