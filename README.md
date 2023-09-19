# https-bench

A naive benchmark of https server for Go vs Rust.

## method

Both https servers use the same self-signed cert and key. The target endpoint simply echo the req body.

## results

```
[lighthouse@VM-20-16-centos https-bench]$ ./atk req.http
Requests      [total, rate, throughput]  1000, 100.10, 99.69
Duration      [total, attack, wait]      10.031519156s, 9.990400793s, 41.118363ms
Latencies     [mean, 50, 95, 99, max]    10.160239ms, 10.454684ms, 10.616548ms, 10.754223ms, 41.118363ms
Bytes In      [total, mean]              1204000, 1204.00
Bytes Out     [total, mean]              1204000, 1204.00
Success       [ratio]                    100.00%
Status Codes  [code:count]               200:1000
Error Set:
```

```
[lighthouse@VM-20-16-centos https-bench]$ ./atk req.http
Requests      [total, rate, throughput]  1000, 100.10, 100.10
Duration      [total, attack, wait]      9.990375038s, 9.990063312s, 311.726µs
Latencies     [mean, 50, 95, 99, max]    363.637µs, 351.781µs, 453.434µs, 558.39µs, 4.333425ms
Bytes In      [total, mean]              1204000, 1204.00
Bytes Out     [total, mean]              1204000, 1204.00
Success       [ratio]                    100.00%
Status Codes  [code:count]               200:1000
Error Set:
```
