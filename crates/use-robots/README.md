# use-robots

Robots and indexing directive primitives for `RustUse`.

## Example

```rust
use use_robots::{FollowDirective, IndexDirective, RobotsDirective};

let directive = RobotsDirective::new(IndexDirective::NoIndex, FollowDirective::NoFollow);

assert_eq!(directive.to_meta_content(), "noindex,nofollow");
```

## Scope

- Robots directives, indexing/follow/snippet/archive labels, bot names, and common combinations.

## Non-goals

- robots.txt crawling, fetching, cache behavior, or bot-specific policy engines.

## License

Licensed under either Apache-2.0 or MIT, at your option.