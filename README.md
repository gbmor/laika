# laika

[![Build Status](https://travis-ci.com/gbmor/laika.svg?branch=master)](https://travis-ci.com/gbmor/laika) [![codecov](https://codecov.io/gh/gbmor/laika/branch/master/graph/badge.svg)](https://codecov.io/gh/gbmor/laika)

Async Gemini protocol server, using `async-std`, `async-tls`, and `rustls`.

This thing isn't finished yet, so there are no tagged releases. If you want to
build from master, beware: **here be dragons**.

* Drops privs (by default, to `laika`)
* Serves static content
* Configurable gemini root, port, ip to bind to, logfile location (`laika --help`)
* Handles failure conditions gracefully

I will be implementing the following soon:

* User directories (`~/public_gemini`)
* CGI (FastCGI? SCGI? CGI?)
* `10 INPUT` responses and replies

### License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE_APACHE](LICENSE_APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE_MIT](LICENSE_MIT) or
   http://opensource.org/licenses/MIT)