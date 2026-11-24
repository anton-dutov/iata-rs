# IATA

[![Build Status](https://travis-ci.org/anton-dutov/iata-rs.svg?branch=master)](https://travis-ci.org/anton-dutov/iata-rs)

## Module: BCBP
Implements a BCBP parser/builder for the boarding pass format described in IATA Resolution 792 - [Version 6](https://www.iata.org/whatwedo/stb/Documents/BCBP-Implementation-Guide-5th-Edition-June-2016.pdf)

Use `BcbpView::parse` to read fixed-width fields with their original padding.
Convert the view into an editable `Bcbp` to normalize strings and validate
typed fields such as the flight day.

```rust
use iata::bcbp::{view::BcbpView, Bcbp, BcbpResult};

fn edit_pass(raw: &str) -> BcbpResult<String> {
    let view = BcbpView::parse(raw)?;
    let mut pass = Bcbp::try_from(view)?;
    pass.set_passenger_name("IVANOV/IVAN")?;
    pass.encode()
}
```

To read directly into the owned model, use `raw.parse::<Bcbp>()?`.
The existing `decode_bcbp` and `encode_bcbp` methods remain available.
The decoder checks the wire layout before converting the fields. If several
fields are malformed, it can report a structural error before an invalid value.
Encoding writes the normalized model, including both nonconsecutive baggage
tags when present. A section that exceeds its two-digit hexadecimal length
returns `Error::FieldLengthExceeded`.

### References
See docs in 'ref' folder
