# Easy Validation of Either IP Addresses or Domains

#### Validate `Domain`
```rust
assert_eq!(true, ValAddr::is_domain("example.com"));
```

#### Validate `IP` Address
```rust
assert_eq!(true, ValAddr::is_ip("192.168.176.43"));
```