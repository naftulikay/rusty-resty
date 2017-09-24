# rusty-resty [![Build Status][travis.svg]][travis]

A [Vagrant][vagrant] environment for running a [Rust][rust] web application proxied by [OpenResty][openresty] with
service discovery via [Consul][consul].

## Scaling

Horizontal scaling is easily possible by simply adding more instances of the Rust web-server. When used with, say, an
Elastic Load Balancer, OpenResty can also scale horizontally.

## Implementation

Everything is implemented in Vagrant on a CentOS 7 VM.

OpenResty is installed with a Consul Lua library for discovering backends. It uses `nginx.timer.at` to poll Consul for
services every 10 seconds.

The Rust web server written is using [Tokio][tokio], and will register itself with Consul upon start-up.

## License

Licensed under the MIT license, see `./LICENSE`.

 [vagrant]: https://vagrantup.com
 [rust]: https://www.rust-lang.org
 [consul]: https://www.consul.io/
 [openresty]: https://openresty.org/
 [tokio]: https://tokio.rs/
 [travis]: https://travis-ci.org/naftulikay/rusty-resty
 [travis.svg]: https://travis-ci.org/naftulikay/rusty-resty.svg
