# `garminfit`

[![Build Status](https://travis-ci.org/jmackie/garminfit.svg?branch=master)](https://travis-ci.org/jmackie/garminfit)

_Flexible and Interoperable Data Transfer_ (FIT) Protocol

WIP library for encoding and decoding Garmin's binary file format.

## Generating SDK code

You can download the latest SDK release from [here][sdk]. If you extract the release
to `./MyFitSDKRelease` and the version is `21.00.00` you can go ahead and run

```
make sdk-modules FIT_SDK_PROFILE=./MyFitSDKRelease FIT_SDK_VERSION=21.00.00
```

## TODO

-   Tests
-   Some kind of streaming/`Reader` interface
-   Use something more efficient than giant match arms ([phf][rust-phf]?)
-   Emulate the FitCSVTool.jar for testing purposes

[sdk]: https://www.thisisant.com/resources/fit
[rust-phf]: https://github.com/sfackler/rust-phf
