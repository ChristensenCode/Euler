from time import time
from numba import njit, prange, vectorize
import numpy as np


@njit
def apply_calc(n):
    if n % 2 == 0:
        return n / 2
    return 3 * n + 1


@vectorize
def calculate_term_number_length(guess_value):
    term_list = 1
    while guess_value != 1:
        guess_value = apply_calc(guess_value)
        term_list += 1
    return term_list


start_time = time()
array_values = np.arange(1, 1_000_001)
answers = calculate_term_number_length(array_values)

solution = np.vstack([array_values, answers]).transpose()
solution = np.where(solution == solution[..., 1].max())[0]

elapsed_time = time() - start_time
print(f"Total Time {elapsed_time}")
print(f"The answer is {solution[1]} with a length of {solution[0]}")
