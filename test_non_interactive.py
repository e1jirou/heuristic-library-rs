# [current directory]
#   ├── index.html
#   ├── test_non_interactive.py # this file
#   ├── [submission file or directory]
#   ├── data
#   │   ├── input.csv
#   │   └── result.csv
#   └── tools
#       ├── in/ # input
#       ├── out/ # output
#       └── target/release/tester # local visualizer


COMPILE_CMD = "cargo build --release"
CLEAN_CMD = "cargo clean"
EXEC_CMD = "./target/release/ahc"

TOOL_DIR = "tools"
TOOL_COMPILE_CMD = "cargo build --release --bin vis"
TOOL_EXEC_CMD = "./tools/target/release/vis"

SCORE_TEXT = "Score ="

IN_DIR = "tools/in"
OUT_DIR = "tools/out"

RES_DIR = "data/result.csv"

START_SEED = 0
NUM_CASES = 100
SEED_STEP = 1

NUM_PARALLELS = 10

TIME_LIMIT_SEC = 2.0


import numpy as np
import concurrent.futures
import csv
import os
import shutil
import subprocess
import sys
import time


def compile():
    cp = subprocess.run(COMPILE_CMD.split())
    if cp.returncode != 0:
        print(cp.stderr)
        print("ERROR: compile failed")
        exit(1)

    cp = subprocess.run(TOOL_COMPILE_CMD.split(), cwd=TOOL_DIR)
    if cp.returncode != 0:
        print(cp.stderr)
        print("ERROR: tester compile failed")
        exit(1)


def clean():
    cp = subprocess.run(CLEAN_CMD.split())
    if cp.returncode != 0:
        print(cp.stderr)
        print("ERROR: clean failed")
        exit(1)


class Result:
    def __init__(self, seed, in_path, score, time_elapsed_sec):
        self.seed = seed
        self.in_path = in_path
        self.score = score
        self.time_elapsed_sec = time_elapsed_sec

    def __str__(self):
        return f"#{self.in_path}: Score = {self.score}, log2(Score) = {np.log2(max(1, self.score)):.3f}, Time = {self.time_elapsed_sec:.3f} sec"


def task(seed, in_path, out_path):
    time_start_sec = time.perf_counter()

    cmd = f"{EXEC_CMD} < {in_path} > {out_path}"
    res = subprocess.run(cmd, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)

    std_err = res.stderr.decode("utf-8", "ignore")
    if std_err:
        print(std_err)

    time_end_sec = time.perf_counter()
    time_elapsed_sec = time_end_sec - time_start_sec
    if time_elapsed_sec > TIME_LIMIT_SEC:
        print(f"WARNING: Time Limit Exceeded ({time_elapsed_sec} sec) in {in_path}")

    res = subprocess.run([TOOL_EXEC_CMD, in_path, out_path], stdout=subprocess.PIPE, stderr=subprocess.PIPE)

    ret = Result(seed, in_path, -1, time_elapsed_sec)
    std_out = res.stdout.decode("utf-8", "ignore")
    for out in std_out.split("\n"):
        if SCORE_TEXT in out:
            ret.score = int(out.split(SCORE_TEXT)[1])
            break
    else:
        print("WARNING: Cannot Find Score")

    return ret


def main():
    compile()

    if not os.path.exists(IN_DIR):
        print(f"ERROR: cannot find {IN_DIR}")
        exit(1)

    if os.path.exists(OUT_DIR):
        print(f"remove {OUT_DIR}")
        shutil.rmtree(OUT_DIR)
    os.mkdir(OUT_DIR)

    executor = concurrent.futures.ProcessPoolExecutor(max_workers=NUM_PARALLELS)
    futures = []
    print(f"start test")
    print(f"{NUM_CASES} cases, {NUM_PARALLELS} parallels")
    for case_id in range(NUM_CASES):
        seed = START_SEED + case_id * SEED_STEP
        in_path = os.path.join(IN_DIR, f"{seed:04}.txt")
        out_path = os.path.join(OUT_DIR, f"{seed:04}.txt")
        futures.append(executor.submit(task, seed, in_path, out_path))

    results = []
    for future in concurrent.futures.as_completed(futures):
        result = future.result()
        print(str(result))
        results.append(result)

    results.sort(key=lambda x: x.seed)
    scores = np.array([result.score for result in results])
    scores = np.where(scores == 0, -1, scores)
    print(f"AC: {np.count_nonzero(scores != -1)} / {NUM_CASES} cases")
    log2scores = np.log2(scores)
    times = np.array([result.time_elapsed_sec for result in results])
    print(f"Score: Avg. {np.mean(scores):.3f}, Min. {np.min(scores):.3f}, Max. {np.max(scores):.3f}")
    print(f"log2(Score): Avg. {np.mean(log2scores):.3f}, Min. {np.min(log2scores):.3f}, Max. {np.max(log2scores):.3f}")
    print(f"Time: Avg. {np.mean(times):.3f}, Min. {np.min(times):.3f}, Max. {np.max(times):.3f}")

    clean()

    if len(sys.argv) == 1:
        print("WARNING: no name (default = 'unknown')")
        name = "unknown"
    else:
        name = sys.argv[1]

    with open(RES_DIR, "a") as f:
        writer = csv.writer(f)
        writer.writerow([name] + list(map(str, scores)))


if __name__ == "__main__":
    main()
