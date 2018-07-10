# difftoml

A small utility to display the difference between two toml files

## Example

```
 ‡ ./target/release/difftoml assets/test_1.toml assets/test_2.toml

Entries only found in assets/test_1.toml
["field1", "name"]: "b"
["int_value"]: 123
["field1", "values"]: [1.23, 4.56, 7.89]

Entries only found in assets/test_2.toml
["field3", "name"]: "b"
["field3", "values"]: [1.23, 4.56, 7.89]
["integer_value"]: 123

Unequal value for key ["name"]
<: "first"
>: "second"
Unequal value for key ["field0", "values"]
<: [0.12, 3.45, 6.78]
>: [0.123, 3.456, 6.789]
```
