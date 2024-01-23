# lalrpop-json
A JSON parser using lalrpop.

```rust

use lalrpop_json::{parse_value, Value};

let value: Value = parse_value(r#"
{
    "hello": "world",
    "array": ["first", 2, true, false, null, { "more": 2 }]
}
"#).unwrap();

```
