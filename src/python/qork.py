import os

# No 'if __name__ == "__main__"' check is really necessary, this is always the
# main method.

os.environ["IN_QORK"] = "1"

print("Running qork.")
