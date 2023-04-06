# paris-log

A crate that allows you to use [`paris`'s formatting](https://github.com/0x20F/paris/blob/master/README.md#color-keys) with the `log` crate.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
paris-log = "1"
```

Instead of using `log` macros:

```rs
use log::{info, error, ...};
```

You can use `paris-log` macros:

```rs
use paris_log::{info, error, ...};
```

Now you can use [`paris`'s formatting](https://github.com/0x20F/paris/blob/master/README.md#color-keys):

```rs
info!("This <cyan>is <bright green>a log<//>! <green><tick></>");
```

If you only need to use `paris`'s formatting a few times, it might be better to use the full path specifier (for example, `paris_log::info`).

`paris-log`'s macros should be usable just like `log` macros; you can use format specifiers like normal. However, they currently don't support manually specifying log target. If you need this feature,
please make an issue / PR! Additionally, `paris-log` does not have the `log` macro from the `log` crate since it seemed repetitive.

> **Note**
>
> You must use a logging implementation along with this crate for logs to show up since `paris-log` uses the `log` crate internally.
> [See here for more info.](https://docs.rs/log/latest/log/#available-logging-implementations) If you don't need the extra features the `log` crate and logging implementations provide, consider using
> `paris` itself.

## The `icons` feature

If you have used `paris`, you may have noticed that logs have a nice icon in front of them to indicate if it is info, error, warn or success. `paris-log` can automatically do this for you if you
enable the `icons` feature:

```toml
[dependencies]
paris-log = { version = "1", features = ["icons"] }
```

```rs
info!("This <cyan>is <bright green>a log<//>!");
// Without `icons` feature: This is a log!
// With `icons` feature: ℹ This is a log!
```

Icons will only be added to the `error`, `warn` and `info` macros. This feature also adds the `success` macro, for feature parity with `paris`. (the `success` macro uses the info log level)
