# h3api

Right now, this is just straight [h3](https://github.com/uber/h3) bindings to Rust using `bindgen`. I added a test
as an example of how it can be used.

In a subsequent iteration, I'd like to try to make safe, `Option`-ized calls to the surface API, though considering
the total depth of `h3` it might make more sense to just RIIR

## Usage

### Prerequisites

- Clone [h3](https://github.com/uber/h3) and follow the installation instructions
- GOTCHA: This has been tested with clang-5.0 only. Bindgen projects have been known to be demanding re: the clang API.

### Including in your project

**Cargo.toml**
```
[dependencies]
h3api = { git = "https://github.com/scottmmjackson/h3api" }
```

**lib.rs**
```
use h3api;

fn main() {
    unsafe {
        let input = GeoCoord {
            lat: degsToRads(95.12345),
            lon: degsToRads(-21.12345),
        };
        let resolution: c_int = 5;
        let output = geoToH3(&input, resolution);
        println!("{:X}", output); // "85056333FFFFFFF", the h3 index of this location at resolution 5
    }
}
```

