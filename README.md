# Rust servers vs bun

Comparing performance of Rust web servers to Bun, a new Zig-based JS all-in-one tool named after a very cute [rabbit](https://twitter.com/kipperrii/status/1574557416741474304).

```
bun --version
0.1.13
rustup --version
rustup 1.25.1 (bb60b1e89 2022-07-12)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.66.0-nightly (8b705839c 2022-09-26)`
```

# Result summary (1)

Using `hey -c 100 -n 5000000 http://localhost:3000`
Using `cargo run --release`

| Bun  | Rocket | Actix (8 threads) | Actix (4 threads) | Actix (TechEmpower config, 8 th) | Hyper | Salvo | May  | Xitca |
| ---- | ------ | ----------------- | ----------------- | -------------------------------- | ----- | ----- | ---- | ----- |
| 41.5 | 53.4   | 54.8              | 44                | 56.45                            | 43.16 | 43.8  | 55.1 | 58.57 |

# Results summary (2)

Using `wrk -c 25 http://localhost:3000`
Using `RUSTFLAGS='-C target-cpu=native' cargo run --release`

- Actix: 153k rps (1 thread)
- Actix: 192k rps (4 threads)
- Bun: 178.4k rps (1 thread)
- Hyper: 173.k rps (1 thread)

# Results

`bun run app.ts`

```
Summary:
  Total:        41.5100 secs
  Slowest:      0.7871 secs
  Fastest:      0.0000 secs
  Average:      0.0041 secs
  Requests/sec: 120452.9266

  Total data:   60000000 bytes
  Size/request: 60 bytes

Response time histogram:
  0.000 [1]     |
  0.079 [999780]        |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.157 [27]    |
  0.236 [83]    |
  0.315 [3]     |
  0.394 [6]     |
  0.472 [4]     |
  0.551 [2]     |
  0.630 [1]     |
  0.708 [11]    |
  0.787 [82]    |


Latency distribution:
  10% in 0.0003 secs
  25% in 0.0005 secs
  50% in 0.0008 secs
  75% in 0.0010 secs
  90% in 0.0012 secs
  95% in 0.0015 secs
  99% in 0.0024 secs

Details (average, fastest, slowest):
  DNS+dialup:   0.0000 secs, 0.0000 secs, 0.7871 secs
  DNS-lookup:   0.0000 secs, 0.0000 secs, 0.2813 secs
  req write:    0.0000 secs, 0.0000 secs, 0.0206 secs
  resp wait:    0.0037 secs, 0.0000 secs, 0.4477 secs
  resp read:    0.0003 secs, 0.0000 secs, 0.0492 secs

Status code distribution:
  [200] 1000000 responses
```

`cargo run --release --bin rocket`

```
Summary:
  Total:        53.4581 secs
  Slowest:      0.8108 secs
  Fastest:      0.0000 secs
  Average:      0.0053 secs
  Requests/sec: 93531.1681

  Total data:   65000000 bytes
  Size/request: 65 bytes

Response time histogram:
  0.000 [1]     |
  0.081 [999751]        |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.162 [61]    |
  0.243 [78]    |
  0.324 [3]     |
  0.405 [6]     |
  0.486 [6]     |
  0.568 [3]     |
  0.649 [9]     |
  0.730 [4]     |
  0.811 [78]    |


Latency distribution:
  10% in 0.0004 secs
  25% in 0.0006 secs
  50% in 0.0009 secs
  75% in 0.0012 secs
  90% in 0.0017 secs
  95% in 0.0021 secs
  99% in 0.0042 secs

Details (average, fastest, slowest):
  DNS+dialup:   0.0000 secs, 0.0000 secs, 0.8108 secs
  DNS-lookup:   0.0000 secs, 0.0000 secs, 0.2036 secs
  req write:    0.0000 secs, 0.0000 secs, 0.0646 secs
  resp wait:    0.0050 secs, 0.0000 secs, 0.4512 secs
  resp read:    0.0002 secs, 0.0000 secs, 0.0455 secs

Status code distribution:
  [200] 1000000 responses
```

`cargo run --release --bin actix`

```
Summary:
  Total:        54.8279 secs
  Slowest:      0.9128 secs
  Fastest:      0.0000 secs
  Average:      0.0054 secs
  Requests/sec: 91194.4513

  Total data:   60000000 bytes
  Size/request: 60 bytes

Response time histogram:
  0.000 [1]     |
  0.091 [998688]        |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.183 [966]   |
  0.274 [222]   |
  0.365 [6]     |
  0.456 [19]    |
  0.548 [16]    |
  0.639 [6]     |
  0.730 [3]     |
  0.822 [57]    |
  0.913 [16]    |


Latency distribution:
  10% in 0.0001 secs
  25% in 0.0001 secs
  50% in 0.0003 secs
  75% in 0.0006 secs
  90% in 0.0014 secs
  95% in 0.0025 secs
  99% in 0.0164 secs

Details (average, fastest, slowest):
  DNS+dialup:   0.0000 secs, 0.0000 secs, 0.9128 secs
  DNS-lookup:   0.0001 secs, 0.0000 secs, 0.2118 secs
  req write:    0.0001 secs, 0.0000 secs, 0.0926 secs
  resp wait:    0.0047 secs, 0.0000 secs, 0.3886 secs
  resp read:    0.0004 secs, 0.0000 secs, 0.0983 secs

Status code distribution:
  [200] 1000000 responses
```

`cargo run --release --bin hyper`

```
Summary:
  Total:        43.1683 secs
  Slowest:      0.7793 secs
  Fastest:      0.0000 secs
  Average:      0.0043 secs
  Requests/sec: 115825.6097

  Total data:   60000000 bytes
  Size/request: 60 bytes

Response time histogram:
  0.000 [1]     |
  0.078 [999896]        |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.156 [0]     |
  0.234 [2]     |
  0.312 [0]     |
  0.390 [1]     |
  0.468 [1]     |
  0.545 [0]     |
  0.623 [2]     |
  0.701 [0]     |
  0.779 [97]    |


Latency distribution:
  10% in 0.0002 secs
  25% in 0.0003 secs
  50% in 0.0006 secs
  75% in 0.0010 secs
  90% in 0.0016 secs
  95% in 0.0022 secs
  99% in 0.0049 secs

Details (average, fastest, slowest):
  DNS+dialup:   0.0000 secs, 0.0000 secs, 0.7793 secs
  DNS-lookup:   0.0000 secs, 0.0000 secs, 0.0797 secs
  req write:    0.0001 secs, 0.0000 secs, 0.0435 secs
  resp wait:    0.0034 secs, 0.0000 secs, 0.5449 secs
  resp read:    0.0005 secs, 0.0000 secs, 0.0708 secs

Status code distribution:
  [200] 1000000 responses
```

`cargo run --release --bin actix_bench`

```
Summary:
Total: 56.4581 secs
Slowest: 0.9353 secs
Fastest: 0.0000 secs
Average: 0.0056 secs
Requests/sec: 88561.2840

Total data: 60000000 bytes
Size/request: 60 bytes

Response time histogram:
0.000 [1] |
0.094 [998818] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
0.187 [772] |
0.281 [163] |
0.374 [111] |
0.468 [33] |
0.561 [7] |
0.655 [27] |
0.748 [16] |
0.842 [36] |
0.935 [16] |

Latency distribution:
10% in 0.0001 secs
25% in 0.0002 secs
50% in 0.0003 secs
75% in 0.0007 secs
90% in 0.0014 secs
95% in 0.0023 secs
99% in 0.0153 secs

Details (average, fastest, slowest):
DNS+dialup: 0.0000 secs, 0.0000 secs, 0.9353 secs
DNS-lookup: 0.0002 secs, 0.0000 secs, 0.4344 secs
req write: 0.0001 secs, 0.0000 secs, 0.1068 secs
resp wait: 0.0047 secs, 0.0000 secs, 0.3820 secs
resp read: 0.0005 secs, 0.0000 secs, 0.1062 secs

Status code distribution:
[200] 1000000 responses

```

`cargo run --release --bin salvo`

```

Summary:
Total: 43.8538 secs
Slowest: 0.8202 secs
Fastest: 0.0000 secs
Average: 0.0044 secs
Requests/sec: 114015.1979

Total data: 60000000 bytes
Size/request: 60 bytes

Response time histogram:
0.000 [1] |
0.082 [999780] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
0.164 [98] |
0.246 [11] |
0.328 [2] |
0.410 [3] |
0.492 [13] |
0.574 [2] |
0.656 [0] |
0.738 [11] |
0.820 [79] |

Latency distribution:
10% in 0.0002 secs
25% in 0.0004 secs
50% in 0.0006 secs
75% in 0.0010 secs
90% in 0.0016 secs
95% in 0.0022 secs
99% in 0.0046 secs

Details (average, fastest, slowest):
DNS+dialup: 0.0000 secs, 0.0000 secs, 0.8202 secs
DNS-lookup: 0.0000 secs, 0.0000 secs, 0.0977 secs
req write: 0.0001 secs, 0.0000 secs, 0.1318 secs
resp wait: 0.0035 secs, 0.0000 secs, 0.5065 secs
resp read: 0.0005 secs, 0.0000 secs, 0.0391 secs

Status code distribution:
[200] 1000000 responses

```

`cargo run --release --bin may`

```

Summary:
Total: 55.1004 secs
Slowest: 0.8185 secs
Fastest: 0.0000 secs
Average: 0.0054 secs
Requests/sec: 90743.4976

Total data: 60000000 bytes
Size/request: 60 bytes

Response time histogram:
0.000 [1] |
0.082 [999089] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
0.164 [685] |
0.246 [77] |
0.327 [47] |
0.409 [1] |
0.491 [1] |
0.573 [1] |
0.655 [5] |
0.737 [5] |
0.819 [88] |

Latency distribution:
10% in 0.0001 secs
25% in 0.0002 secs
50% in 0.0004 secs
75% in 0.0007 secs
90% in 0.0014 secs
95% in 0.0022 secs
99% in 0.0118 secs

Details (average, fastest, slowest):
DNS+dialup: 0.0000 secs, 0.0000 secs, 0.8185 secs
DNS-lookup: 0.0002 secs, 0.0000 secs, 0.1780 secs
req write: 0.0001 secs, 0.0000 secs, 0.0940 secs
resp wait: 0.0045 secs, 0.0000 secs, 0.5713 secs
resp read: 0.0005 secs, 0.0000 secs, 0.1721 secs

Status code distribution:
[200] 1000000 responses

```

`cargo run --release --bin xitca`

```

Summary:
Total: 57.5772 secs
Slowest: 0.8706 secs
Fastest: 0.0000 secs
Average: 0.0056 secs
Requests/sec: 86839.8576

Total data: 60000000 bytes
Size/request: 60 bytes

Response time histogram:
0.000 [1] |
0.087 [998687] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
0.174 [1176] |
0.261 [24] |
0.348 [3] |
0.435 [9] |
0.522 [9] |
0.609 [3] |
0.696 [2] |
0.784 [6] |
0.871 [80] |

Latency distribution:
10% in 0.0001 secs
25% in 0.0001 secs
50% in 0.0003 secs
75% in 0.0006 secs
90% in 0.0012 secs
95% in 0.0020 secs
99% in 0.0176 secs

Details (average, fastest, slowest):
DNS+dialup: 0.0000 secs, 0.0000 secs, 0.8706 secs
DNS-lookup: 0.0002 secs, 0.0000 secs, 0.2043 secs
req write: 0.0001 secs, 0.0000 secs, 0.1075 secs
resp wait: 0.0050 secs, 0.0000 secs, 0.5173 secs
resp read: 0.0003 secs, 0.0000 secs, 0.1047 secs

Status code distribution:
[200] 1000000 responses

```
