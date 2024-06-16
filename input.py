DATA_DIR = "data"

IN_DIR = "tools/in"

FEATURE_VALUES = ["n", "m"] # TODO


import os
import csv


class Input:
    def read_input(self, file_name):
        self.features = [file_name[:4], file_name[:4]]

        with open(os.path.join(IN_DIR, file_name), "r") as f:
            # TODO
            ...


def main():
    if not os.path.exists(DATA_DIR):
        os.mkdir(DATA_DIR)

    inputs = []
    for file_name in sorted(os.listdir(IN_DIR)):
        input = Input()
        input.read_input(file_name)
        inputs.append(input)

    with open(os.path.join(DATA_DIR, "input.csv"), "w") as f:
        writer = csv.writer(f)
        writer.writerow(["file", "seed"] + FEATURE_VALUES)
        for input in inputs:
            writer.writerow(input.features)


if __name__ == "__main__":
    main()
