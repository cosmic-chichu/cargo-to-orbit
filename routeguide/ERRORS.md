- 
```
error: failed to parse manifest at `/Users/srao12/play/rust/trpl-examples/routeguide/Cargo.toml`

Caused by:
no targets specified in the manifest
either src/lib.rs, src/main.rs, a [lib] section, or [[bin]] section must be present
```
-
```
error[E0277]: the size for values of type `[Feature]` cannot be known at compilation time
   --> src/server.rs:96:23
    |
96  |       let route_guide = RouteGuideService {
    |  _______________________^
97  | |         features = [
98  | |             Feature{
99  | |                 location: Some(Point{
...   |
112 | |         ];
113 | |     };
    | |_____^ doesn't have a size known at compile-time
    |
    = help: within `RouteGuideService`, the trait `Sized` is not implemented for `[Feature]`, which is required by `RouteGuideService: Sized`
note: required because it appears within the type `RouteGuideService`
   --> src/server.rs:15:8
    |
15  | struct RouteGuideService {
    |        ^^^^^^^^^^^^^^^^^
    = note: structs must have a statically known size to be initialized

```