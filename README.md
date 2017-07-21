# stm32f429x

Peripheral Access API built with [svd2rust](https://github.com/japaric/svd2rust).

## TODO

I have plans on (once I can actually get the basic annotations all there)
actually creating smarter functions that can handle multiple pins on some
things being read/written at the same time.

Especially as Rust continues to add new support for more basic C-like
constructs (unions!), it'll be nice to have a cleaner interface.

However, the better choice is to leverage the
[f4](https://github.com/dkarwowski/f4) package, which is an example of a much
cleaner use of this library, abstracting away the bits themselves.
