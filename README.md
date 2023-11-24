# Asserter

[`asserter`](/contracts/asserter/) is a CosmWasm smart contract that allow to execute any `QueryRequest`, read a field of any `QueryResponse`, add the value of the read field to the tx log, and if specified, assert this value.

The use cases of these contracts are mainly two: 

- **Assert field value of any QueryResponse**: Immagine that swap on liquidity pool doesn't provide a `max_spread` or `min_return` option.
How can a swap been reverted if the output amount is lower then expected? With assert contract is possible to add an extra Msg into the original tx that basically query a token balance for the user that have to receive the swap output, and assert if the balance is greater than expected value.
If it's not, whole tx is reverted.
You can apply this logic to any kind of tx.

- **Debug and Simulation**: Simulate tx is a really powerfull tools since allow to analize `logs` without change the state of the chain.
However one of the biggest limitations is the possibility to perform queries after the simulation, to check any changes made from the messages of tx.
To solve this, tou can append a msg to your tx to this contract, specify a list of query and the filed of thw QueryResponse that you want to append to the log.
If you don't want to assert the value but just add to the tx log the request, leave the `assert_with` field as None.

## How to use

```rust
pub struct QueryToAssert {
    pub request: QueryRequest<Empty>,
    pub path_key: Option<Vec<PathKey>>,   
    pub assert_with: Option<AssertInfo>,
}
```

- `request`: `QueryRequest` to be execute
- `path_key`: In case `QueryResult` is a struct an not a value, you can specify a `PathKey` that indicate to the contract the location of the field to read.

## **PathKey**

```rust
pub struct PathKey {
    pub key_type: KeyType,
    pub value: String,
}

pub enum KeyType {
    ArrayIndex,
    String,
}
```

- `key_type`: Indicate if the key is a `json` key as `String` or a `ArrayIndex`;
- `value`: is the name of the key, or the index of the array.

### Example:

`QueryResponse` struct

```json
{
    "key_1": "str_value_1",
    "key_2": "str_value_2",
    "key_3": {
        "key_4": "str_value_4",
        "key_5": 5
    }
}
```

If you want to read the value of `key_5`, the `path_key` field will be:

```rust
let path_key = Some(vec![
    PathKey{ key_type: KeyType::String, value: "key_3"},
    PathKey{ key_type: KeyType::String, value: "key_5"}
])
```

## **AssertInfo**

```rust
pub struct AssertInfo {
    pub data_type: DataType,
    pub value: String,
    pub operator: AssertOperator,
}

pub enum DataType {
    Int,
    String,
    Decimal,
}

pub enum AssertOperator {
    Lesser,
    LesserEqual,
    Equal,
    Greater,
    GreaterEqual,
}
```

### Example (assert the value of `key_5` = `5`):

```rust
let assert_info = AssertInfo{
    data_type: DataType::Int.
    value: "5"
    operator = AssertOperator::Equal
}
```

in the example, `operator` fails if is equal to `Greater`



