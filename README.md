# DataFusion in Ruby

This is yet another Ruby library that binds to [Apache Arrow](https://arrow.apache.org/) in-memory query engine [DataFusion](https://github.com/apache/arrow-datafusion).

This is an alternative to [datafuion-contrib/datafusion-ruby](https://github.com/datafusion-contrib/datafusion-ruby). Please refer to FAQ below.

## Quick Start

Gemfile
```
gem "arrow-datafusion"
```

App
```ruby
require "datafusion"

ctx = Datafusion::SessionContext.new
ctx.register_csv("csv", "test.csv")
ctx.sql("SELECT * FROM csv").collect
```

## Supported features

SessionContext
- [x] new
- [x] register_csv
- [x] sql
- [ ] register_parquet
- [ ] register_record_batches
- [ ] register_udf

Dataframe
- [x] new
- [x] collect
- [ ] schema
- [ ] select_columns
- [ ] select
- [ ] filter
- [ ] aggregate
- [ ] sort
- [ ] limit
- [ ] show
- [ ] join
- [ ] explain

## Contribution Guide

Please see [Contribution Guide](CONTRIBUTING.md).

## FAQ

### Why Magnus?

As of 2022-07, there are a few popular Ruby bindings for Rust, [Rutie](https://github.com/danielpclark/rutie), [Magnus](https://github.com/matsadler/magnus) and [other alternatives](https://github.com/matsadler/magnus#alternatives). Magnus is picked because its API seems cleaner and it seems more clear about safe vs unsafe. The author of Magnus have a "maybe bias" comparison in this [reddit thread](https://www.reddit.com/r/ruby/comments/uskibb/comment/i98rds4/?utm_source=share&utm_medium=web2x&context=3). It is totally subjective and it should not be large effort if we decides to switch to different Ruby bindings fr Rust in future.

### Why the module name and gem name are different?

The module name `Datafusion` follows the [datafusion](https://github.com/apache/arrow-datafusion) and [datafusion-python](https://github.com/datafusion-contrib/datafusion-python). The gem name `datafusion` [is occupied in rubygems.org at 2016](https://rubygems.org/gems/datafusion), so our gem is called `arrow-datafusion`.

Similarly to the Ruby bindings of Arrow, its gem name is called [red-arrow](https://github.com/apache/arrow/tree/master/ruby/red-arrow) and the module is called `arrow`.

### Why another Ruby bindings for Arrow DataFusion?

[datafuion-contrib/datafusion-python](https://github.com/datafusion-contrib/datafusion-python) was the first bindings of Arrow Datafusion (Rust). It was implemented using [pyo3](https://github.com/PyO3/pyo3) for `Rust -> Python`. Besides Python, Datafusion Community also want to have Java and other language bindings. In order to share development resource, [datafuion-contrib/datafusion-c](https://github.com/datafusion-contrib/datafusion-c) is created and be used in [datafuion-contrib/datafusion-ruby](https://github.com/datafusion-contrib/datafusion-ruby). This `Rust -> C -> Ruby/Python/Java/etc` implementation is published as gem "red-datafusion" and couple with "red-arrow".

Around similar time when "red-datafusion" is created, I want to use Arrow DataFusion in Ruby, mainly to query Object Store like S3/GCS, so I create a `Rust -> Ruby` bindings using [Magnus](https://github.com/matsadler/magnus). So I just keep this `Rust -> Ruby` implementation as an alternative and publish it as gem `arrow-datafusion`. To keep it simple, "arrow-datafusion" does not couple with "red-arrow" at the moment.

ps: Datafusion Python was coupled with PyArrow. There is a proposal to separate them in medium to long term. For detail, please refer to [Can datafusion-python be used without pyarrow?](https://github.com/datafusion-contrib/datafusion-python/issues/22).
