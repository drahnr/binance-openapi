# binance-openapi

Based on an (adapted) OpenAPI spec from https://github.com/binance/binance-api-swagger

## Adaptations

* disabled some API calls due to
  - usage of both lower case `m` and upper case `M` breaks with the used generator
  - usage of arrays in args, which doesn't have defined format and hence fails to generate for now

## Motivation

Trying to use binance's rust API leaves much to be desired, the return types are all raw streams
and only provide conversion to strings which have to be manually parsed using i.e. `serde_json` to
hand written types. We can do better.