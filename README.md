# vergen

Generates 3 functions for use in version strings.

```rust
pub fn sha() -> &'static str {
   // Output of 'git rev-parse HEAD'
}
```