# laika

[![builds.sr.ht status](https://builds.sr.ht/~gbmor/laika.svg)](https://builds.sr.ht/~gbmor/laika?) [![Build Status](https://travis-ci.com/gbmor/laika.svg?branch=master)](https://travis-ci.com/gbmor/laika)

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
* Client cert verification

### Development

I'm primarily using [sr.ht/~gbmor/laika](https://sr.ht/~gbmor/laika) for development,
but the tree will be mirrored to [github.com/gbmor/laika](https://github.com/gbmor/laika).

* Send patches to: [~gbmor/laika@lists.sr.ht](mailto:~gbmor/laika@lists.sr.ht) 
* Bug tracker: [https://todo.sr.ht/~gbmor/laika](https://todo.sr.ht/~gbmor/laika)

### Notes

* Gemini project homepage: [https://gemini.circumlunar.space/](https://gemini.circumlunar.space/)
* Spec: [https://gemini.circumlunar.space/docs/specification.html](https://gemini.circumlunar.space/docs/specification.html)
