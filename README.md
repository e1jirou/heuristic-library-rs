# Template for AHC
## Usage
1. Place tools.
1. Edit `src/main.rs`, `scripts/input.py`, and `data/result.csv`.
1. Make `data/input.csv`.
    ```bash
    python3 scripts/input.py
    ```
1. Execute the runner.
    ```bash
    python3 scripts/test_interactive.py [solution name] | tee out.txt
    ```
    or
    ``` bash
    python3 scripts/test_non_interactive.py [solution name] | tee out.txt
    ```
1. Start a simple HTTP server.
    ```bash
    python3 -m http.server 8000
    ```
1. See the standings. [Click here](http://[::]:8000/?contest=data). If the updates are not reflected, please perform a hard reload by pressing Ctrl + Shift + R (Windows/Linux) or Cmd + Shift + R (Mac).

## Links
[Official Usage of Standing Tool](https://img.atcoder.jp/ahc_standings/usage.html)
