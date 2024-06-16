# Template for AHC
## Usage
1. Place tools.
1. Edit `src/main.rs`, `input.py`, and `data/result.csv`.
1. Make `data/input.csv`.
    ```bash
    python3 input.py
    ```
1. Execute the runner.
    ```bash
    python3 test_interactive.py [solution name]
    ```
    or
    ``` bash
    python3 test_non_interactive.py [solution name]
    ```
1. Start a simple HTTP server.
    ```bash
    python3 -m http.server 8000
    ```
1. See the standings. [Click here](http://[::]:8000/?contest=data).

## Links
[Official Usage of Standing Tool](https://img.atcoder.jp/ahc_standings/usage.html)
