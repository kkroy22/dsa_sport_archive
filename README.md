# Dsa_Sport

this crate is not intended to be used as a library for any of the application. this crate only serves some basic concept related to data structure.

## Usage

```rust
use dsa_sport::datastruct::list_struct::LinkedList;
let mut list = LinkedList::new();
list.add_node(1);
list.add_node(2);
list.add_node(3);
assert_eq!(format!("{:?}",list), format!("1 -> 2 -> 3 -> x"));
assert_eq!(list.len(), 3);
```

## Contributing
Please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## Version
`The Greek and Latin words for leg and foot have given English the combining form -pod`

