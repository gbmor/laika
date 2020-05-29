# laika

[![Build Status](https://travis-ci.com/gbmor/laika.svg?branch=master)](https://travis-ci.com/gbmor/laika) [![codecov](https://codecov.io/gh/gbmor/laika/branch/master/graph/badge.svg)](https://codecov.io/gh/gbmor/laika)

Async Gemini protocol server, using `async-std`, `async-tls`, and `rustls`.

This thing isn't finished yet, so there are no tagged releases. If you want to
build from master, beware: **here be dragons**. I also need to do some cleanup
and reorganization.

* Drops privs (by default, to `laika`)
* Serves static content
* Configurable gemini root, port, ip to bind to, logfile location (`laika --help`)
* Handles failure conditions gracefully

I will be implementing the following soon:

* User directories (`~/public_gemini`)
* CGI (FastCGI? SCGI? CGI?)
* `10 INPUT` responses and replies
