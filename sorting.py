import time
from random import randint
import tracemalloc


# Bubble Sort algorithm implementation in Python
def bubble_sort(array):
    n = len(array)

    for i in range(n):
        is_sorted = True

        for j in range(n - i - 1):
            if array[j] > array[j + 1]:
                array[j], array[j + 1] = array[j + 1], array[j]
                is_sorted = False
        if is_sorted:
            break
    return array


# Generate an array of `array_len` items consisting
# of random integer values between 0 and 999
array_len = 10000
array = [randint(0, 1000) for i in range(array_len)]

if __name__ == "__main__":
    start_time = time.time()
    tracemalloc.start()
    bubble_sort(array)
    end_time = time.time()
    current, peak = tracemalloc.get_traced_memory()
    tracemalloc.stop()

    with open("results.txt", "w") as result_file:
        result_file.write(
            "Bubble Sort implementation in Python took: "
            + f"{end_time - start_time} seconds to execute\n"
        )
        result_file.write(f"Current memory usage: {peak/1024} KB")

    print(
        "Bubble Sort implementation in Python took: {} "
        "seconds to execute".format((end_time - start_time))
    )
    print("Current memory usage: {} KB".format(peak / 1024))
