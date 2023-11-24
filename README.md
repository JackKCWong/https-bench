# https-bench

A naive benchmark of https server for Go vs Rust.

## method

Both https servers use the same self-signed cert and key. The target endpoint simply echo the req body.

## results

```
[lighthouse@VM-20-16-centos https-bench]$ (cd rust-https && cargo run)
[lighthouse@VM-20-16-centos https-bench]$ ./atk req.http
Requests      [total, rate, throughput]  30000, 100.00, 99.99
Duration      [total, attack, wait]      5m0.030966866s, 4m59.989786803s, 41.180063ms
Latencies     [mean, 50, 95, 99, max]    10.140154ms, 10.02219ms, 10.992857ms, 11.35211ms, 41.180063ms
Bytes In      [total, mean]              36120000, 1204.00
Bytes Out     [total, mean]              36120000, 1204.00
Success       [ratio]                    100.00%
Status Codes  [code:count]               200:30000
Error Set:
```

```
[lighthouse@VM-20-16-centos https-bench]$ (cd go-https && go run main.go)
[lighthouse@VM-20-16-centos https-bench]$ ./atk req.http
Requests      [total, rate, throughput]  30000, 100.00, 100.00
Duration      [total, attack, wait]      4m59.990345346s, 4m59.989949216s, 396.13µs
Latencies     [mean, 50, 95, 99, max]    397.004µs, 382.491µs, 496.136µs, 808.992µs, 17.974324ms
Bytes In      [total, mean]              36120000, 1204.00
Bytes Out     [total, mean]              36120000, 1204.00
Success       [ratio]                    100.00%
Status Codes  [code:count]               200:30000
Error Set:
```
