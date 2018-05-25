# Hello actix-web

[![Build Status](https://travis-ci.org/mmacedoeu/hello.svg?branch=master)](https://travis-ci.org/mmacedoeu/hello)
[![Language (Rust)](https://img.shields.io/badge/powered_by-Rust-blue.svg)](http://www.rust-lang.org/)

Hello is a "hello-world" starter kit that counter the number of occurences of a Grapheme and return it order descending by number of occurences:

## Features

* Async/Sync [actors](https://github.com/actix/actix).
* Actor communication in a local/thread context.
* Uses [Futures](https://crates.io/crates/futures) for asynchronous message handling.
* HTTP1/HTTP2 support ([actix-web](https://github.com/actix/actix-web))
* Typed messages (No `Any` type).
* Argument parsing with [clap](https://github.com/kbknapp/clap-rs)
* App data directory handling with [app-dirs](https://github.com/AndyBarron/app-dirs-rs)
* Error handling with [derive-error-chain](https://github.com/Arnavion/derive-error-chain)
* REST interface
* CORS enabled

### Not Featured

* Json result paging
* Authentication
* Json error handling

## Install

To compile and install you need to first install Rust [compiler](https://www.rust-lang.org/en-US/install.html)

`curl https://sh.rustup.rs -sSf | sh`

Compile for release

`RUSTFLAGS="-C target-cpu=native" cargo build --release`

### Platform support

Should compile and work on all rust compiler supported [plataforms](https://forge.rust-lang.org/platform-support.html) but only tested for 64bit linux

### Docker support

To build image

`docker build -t hello .`

There is a docker image on docker hub

`docker pull mmacedoeu/hello`

`docker run -p 8080:8080 mmacedoeu/hello`

### Snapd support

Look for Snap install [instructions](https://docs.snapcraft.io/core/install) for your OS and install swapi with:

`sudo snap install hello --channel=edge --devmode`

### Cloud support

Support for heroku based Cloud services with provided Procfile, should work on any cloud provider based on Heroku Procfile and with minor manifest files on any Cloud provider

## Usage

Display help:

`./target/release/hello --help`

```txt
hello

USAGE:
    hello [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b <base>               Specify the base storage path.
    -i <IP>                 Specify the hostname portion of the REST API server, IP should be an interface's IP address,
                            or all (all interfaces) or local. [default: local]
    -l <LOG_PATTERN>        Sets a custom logging
    -p <PORT>               Api tcp listener port, default to environment variable PORT or 8080
```

Run with full trace:

`./target/release/hello -l trace`

Run with no logging:

`./target/release/hello -l warn,actix_web::middleware::logger=warn`

## Testing

### Manual Testing

Manual Testing is done with your preferred http client cli like [curl](https://github.com/curl/curl), [Httpie](https://github.com/jakubroztocil/httpie), [http-prompt](https://github.com/eliangcs/http-prompt), or any http test tool like [postman](https://www.getpostman.com/)

Process count on input

`http :8080/?input=%E6%97%A5%E6%9B%9C%E6%97%A5HANNAH%EF%BD%A1%E2%97%95%E2%80%BF%E2%97%95%EF%BD%A1NULL`

### Load testing

Using [Vegeta](https://github.com/tsenart/vegeta) with 8 cores during 10 seconds and 10000 requests per second

`echo "GET http://127.0.0.1:8080/?input=%E6%97%A5%E6%9B%9C%E6%97%A5HANNAH%EF%BD%A1%E2%97%95%E2%80%BF%E2%97%95%EF%BD%A1NULL" | vegeta attack -duration=10s -rate=10000 | tee results.bin | vegeta report -reporter=plot > plot.html`

![alt text](https://github.com/mmacedoeu/hello/raw/master/vegeta-plot.png "Read All Latency plot")

`echo "GET http://127.0.0.1:8080/?input=%E6%97%A5%E6%9B%9C%E6%97%A5HANNAH%EF%BD%A1%E2%97%95%E2%80%BF%E2%97%95%EF%BD%A1NULL" | vegeta attack -duration=10s -rate=10000 | tee results.bin | vegeta report`

```txt
Requests      [total, rate]            100000, 10000.10
Duration      [total, attack, wait]    10.000061991s, 9.999899931s, 162.06µs
Latencies     [mean, 50, 95, 99, max]  168.989µs, 143.22µs, 183.443µs, 628.703µs, 14.519712ms
Bytes In      [total, mean]            9100000, 91.00
Bytes Out     [total, mean]            0, 0.00
Success       [ratio]                  100.00%
Status Codes  [code:count]             200:100000  
Error Set:
```
Using wrk:

`wrk -t8 -c200 -d10s http://127.0.0.1:8080/?input=%E6%97%A5%E6%9B%9C%E6%97%A5HANNAH%EF%BD%A1%E2%97%95%E2%80%BF%E2%97%95%EF%BD%A1NULL`

```txt
Running 10s test @ http://127.0.0.1:8080/?input=%E6%97%A5%E6%9B%9C%E6%97%A5HANNAH%EF%BD%A1%E2%97%95%E2%80%BF%E2%97%95%EF%BD%A1NULL
  8 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.41ms    1.37ms  26.89ms   92.66%
    Req/Sec    20.33k     1.99k   42.22k    81.34%
  1626866 requests in 10.10s, 330.47MB read
Requests/sec: 161073.75
Transfer/sec:     32.72MB

```