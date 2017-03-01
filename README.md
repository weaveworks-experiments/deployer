# deployer

Experimental microservice to deploy code
with [flux](https://github.com/weaveworks/flux) in response to alerts.

[Design doc](https://docs.google.com/document/d/1W_ZXdukT6PjKPKASy-7uLzHl-oAe0NQU13usvmBgn4g/edit)


## Implementation notes

Experimenting with [Rust](https://rust-lang.org). Stuff you might want to read:

* [Rust documentation](https://doc.rust-lang.org/book/)
* The [Rust book](https://doc.rust-lang.org/)


Other stuff that might be useful:

* Example http api: https://github.com/brson/httptest
* Datetime library: https://docs.rs/chrono/0.3.0/chrono/index.html
* Serialize / deserialize: https://docs.serde.rs/serde/index.html
* example handler: https://github.com/gnunicorn/clippy-service/blob/master/src/handlers.rs
* prometheus lib: https://docs.rs/prometheus/0.2.8/prometheus/enum.Error.html
  * PR to add protobuf exporter to prometheus lib: https://github.com/pingcap/rust-prometheus/pull/97
* logging library: http://ironframework.io/doc/log/index.html
