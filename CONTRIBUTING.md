# Introduction

datafusion-ruby follows the standard Ruby gem structure. It uses [Magnus](https://github.com/matsadler/magnus) as the Ruby bindings for Rust.

```
|
|- lib # Ruby code
|- ext # Rust code
```

# Develop

## setup

```
make install
```

## Test

```
# compile rust and run ruby tests
make test
```

## Formating

```
make fmt
```
