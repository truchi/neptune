# Neptune's assignment

Written in Rust. 💖 🦀.

You need [`cargo`](https://doc.rust-lang.org/cargo) to run the code.

## Run the server

Run on default `3000` port:

```sh
cargo run --release
```

You can specify the port to run on:

```sh
cargo run --release -- 8080
```

## Run the tests

```sh
cargo test
```

## Run the time example

This example times POST and GET requests.

```sh
cargo run --release --example time
```

With port:

```sh
cargo run --release --example time -- 8080
```

## Remarks

Note that there is ambiguity here:

> The time complexity for calculating stats should be better than O(n). O(n) complexity is insufficient for this task.

N could be one of:

- the total amount of values for a symbol (usually the case in coding challenges)
- the batch size of a `POST /add_batch` request
- `10^8` in a `POST /add_batch` request
- `10^k` in a `GET /stats` request

Also, we used `f64` for prices, which can lead to imprecisions with large prices or as time goes.
Real world finance softwares use prices in cents to work around the issue.

## Insights

To limit complexity, we use:

- A sliding window to store values
- Deques for buffers
- A monotonic deque for min and max
- A running sum for average
- The other formula for variance

A sliding window ensures we do not burn all the host's available memory.
Using deques allows for constant time indexing and operations on both end on the queue.

Monotonic deques keeps track of possible min/max values of the window.
The time complexity is `O(n)` in theory but much faster in practice. Same for space complexity.

Using `var = 1/n * SUM(xi) - avg^2` instead of the usual `var = 1/n * SUM((xi - avg)^2)`
allows us to compute the variance as the window slides.

All the above unlocks computing stats on the fly. Those computations are done when posting data.
Nothing complex really happens when getting the stats.

Even though it was stated otherwise, we implemented the case of concurrent requests for a given symbol.
We believe our server's state would block as short as possible in this case, but this is untested.

## Dependencies

We tried to use as little dependencies as possible:

- [`axum`](docs.rs/axum) for the web server
- [`serde`](docs.rs/serde) for (de)serialization (can do without but impractical)
- [`tokio`] for the async runtime

Tests/Examples dependencies:

- [`rand`] to generate random prices
- [`reqwest`] to send requests to the server

## Improvements

We did not aggressively optimized the code, even though we thought about a few things
to optimize time and space, without complexity impacts.
(Stats are overlapping, sums could be batched, ...)

As discussed above, we could use integer for prices in cents,
allowing us to use floats only in the final stage of the average/variance computations.
If prices are found to be "large", we could use a bigint crate to make sure `SUM(x^2)` fits.
