# Roadmap: системный программист на Rust

Маршрут: **База → Cloud/Infrastructure → ОС и виртуализация → Embedded**

Логика: этап 1 даёт первую работу, этап 2 — глубину и топовые команды, этап 3 — опциональное расширение с форой в 90% навыков.

---

## Этап 0. База (2–4 мес)

### Rust продвинутый
- [ ] Lifetimes в сложных случаях, HRTB
- [ ] Smart pointers: `Box`, `Rc`, `Arc`, `RefCell`, `Cow`
- [ ] Unsafe Rust: raw pointers, FFI, инварианты — *Rustonomicon*
- [ ] Многопоточность: `Send`/`Sync`, atomics, memory ordering — *Rust Atomics and Locks* (Mara Bos, бесплатно онлайн)
- [ ] Async вглубь: устройство Future, pin, executor — туториал tokio mini-redis

### C и низкий уровень
- [ ] C: K&R или *Modern C* (1–2 мес)
- [ ] Проект: свой malloc
- [ ] Проект: простой shell
- [ ] CS:APP — главы про память, линковку, ассемблер
- [ ] Привычка: смотреть свой Rust-код в Godbolt (читать ассемблер, не писать)

### ОС-основы
- [ ] *OSTEP* (Operating Systems: Three Easy Pieces, бесплатно): процессы, потоки, syscalls, память

**Чекпоинт:** свободно читаю C, понимаю вывод Godbolt, написал многопоточную программу с atomics.

---

## Этап 1. Cloud + Infrastructure (6–9 мес) → первая работа

### Сети (2–3 мес)
- [ ] TCP/UDP, сокеты — *Beej's Guide to Network Programming*
- [ ] TLS, HTTP/1.1 → HTTP/2 → HTTP/3 (QUIC), DNS
- [ ] epoll / io_uring — как tokio работает под капотом
- [ ] Проект: HTTP-сервер на голых сокетах
- [ ] Проект: reverse proxy (вдохновляться Pingora от Cloudflare)

### Linux (параллельно)
- [ ] Namespaces, cgroups — фундамент контейнеров
- [ ] strace, perf, основы eBPF (bpftrace)
- [ ] Проект: мини-контейнер-рантайм на Rust

### Хранилища (2–3 мес)
- [ ] LSM-деревья, B-деревья, WAL, MVCC
- [ ] PingCAP Talent Plan (курс на Rust)
- [ ] Проект: KV-хранилище с персистентностью
- [ ] Читать: *DDIA* (2-е издание, ~глава в 1–2 недели)
- [ ] Читать исходники: redb, sled

### Распределённые системы (2–3 мес)
- [ ] MIT 6.824 (лекции бесплатны)
- [ ] Raft — разобрать алгоритм
- [ ] **Проект-визитка:** KV-хранилище с Raft-репликацией

### Инструменты (по ходу)
- [ ] Docker + Kubernetes (уверенное пользование + устройство)
- [ ] gRPC, Protobuf
- [ ] Prometheus, OpenTelemetry
- [ ] flamegraph, tokio-console
- [ ] Один облачный провайдер (AWS — самый частый в вакансиях)

### Фоном
- [ ] Купить RP2040 (~$5), мигать светодиодами с Embassy по выходным

**Чекпоинт:** 2–3 проекта на GitHub, дочитан DDIA, откликаюсь на вакансии (джун/мидл в инфраструктуре).

---

## Этап 2. ОС и виртуализация (год 2–3, параллельно работе)

### Своя ОС
- [ ] Блог-серия *Writing an OS in Rust* (Philipp Oppermann) целиком
- [ ] Планировщик задач (round-robin → приоритеты)
- [ ] Userspace: syscalls, ring 0 → ring 3
- [ ] Простая файловая система (FAT32)
- [ ] Драйвер (клавиатура, virtio или AHCI)
- [ ] Читать xv6 (учебная ОС на C)
- [ ] Справочник: OSDev Wiki

### Виртуализация (мост между работой и ОС)
- [ ] KVM API: простейший гипервизор на Rust через /dev/kvm
- [ ] virtio: как гость общается с хостом
- [ ] Читать исходники: Firecracker, Cloud Hypervisor

### eBPF вглубь
- [ ] aya (Rust-фреймворк для eBPF)

**Чекпоинт:** своя мини-ОС с userspace, работающий toy-гипервизор, понимаю код Firecracker.

---

## Этап 3. Embedded (год 3+, если интерес сохранится)

- [ ] Железо: STM32 (Nucleo) или RP2040
- [ ] *The Embedded Rust Book* + *Discovery Book*
- [ ] Стек: PAC → HAL → BSP, трейты `embedded-hal`
- [ ] Прерывания, таймеры, DMA
- [ ] Протоколы с реальными датчиками: UART, SPI, I2C
- [ ] Embassy (async) и RTIC (realtime)
- [ ] Отладка: probe-rs, defmt, GDB через SWD
- [ ] **Проект-визитка:** драйвер чипа как крейт на crates.io или USB-устройство (HID)

---

## Сквозные привычки

- [ ] Читать чужой код постоянно: tokio → TiKV → Firecracker → Embassy
- [ ] Контрибуции в open source с этапа 1 (TiKV, Vector, Quickwit)
- [ ] Вести заметки/блог о проектах

## Ключевые книги

| Книга | Этап |
|---|---|
| Rustonomicon | 0 |
| Rust Atomics and Locks | 0 |
| CS:APP | 0 |
| OSTEP | 0 |
| Beej's Guide to Network Programming | 1 |
| DDIA (2-е изд.) | 1 |
| The Embedded Rust Book + Discovery | 3 |
