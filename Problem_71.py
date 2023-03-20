"""
By listing the set of reduced proper fractions for d ≤ 1,000,000 in 
ascending order of size, find the numerator of the fraction immediately to the left of 3/7.
"""

from math import gcd
from time import time
from pprint import pprint
from tqdm import tqdm
import numpy as np
from numba import njit


def naive_solution(limit_number: int, target_value: float) -> int:
    """
    Nested loop for numerator and denominator that naively loops over
    all the different combinations, which could mean 1,000,000,000 iterations.

    This should technically should get you the answer, but it will take a long
    time.

    :param limit_number: range of values to check
    :type limit_number: int
    :param target_value: value looking to the left of
    :type target_value: float
    :return: numerator of value left of target.
    :rtype: int
    """

    min_difference = 1000
    for denominator in tqdm(range(1, limit_number + 1)):
        # i could remove this inner loop potential with array operations.
        for numerator in range(1, limit_number + 1):
            decimal_representation = numerator / denominator

            if gcd(numerator, denominator) != 1:
                continue
            elif decimal_representation >= target_value:
                break

            differencer = abs(decimal_representation - target_value)
            if differencer < min_difference:
                min_difference = differencer

    return numerator


@njit
def farey_sequence_jitted(max_denominator: int, target_value: float = 3 / 7) -> int:
    """
    Same as the naive solution, but uses the farey sequence math principle
    https://en.wikipedia.org/wiki/Farey_sequence (Farey neighbours) and
    this function uses numba's njit decorator to speed it up.

    :param max_denominator: maximum denominator value
    :type max_denominator: int
    :param target_value: value to look left of, defaults to 3/7
    :type target_value: float, optional
    :return: numerator of left-most value
    :rtype: int
    """
    first_term = np.array([0, 1])
    second_term = np.array([1, 1])
    while True:
        combined_term = first_term + second_term
        fraction_combined_term = combined_term[0] / combined_term[1]
        if combined_term[1] > max_denominator:
            break

        if fraction_combined_term >= target_value:
            second_term = combined_term
        elif fraction_combined_term < target_value:
            first_term = combined_term

    return first_term[0]


def farey_sequence_normal(max_denominator: int, target_value: float = 3 / 7) -> int:
    """
    Same as the naive solution, but uses the farey sequence math principle
    https://en.wikipedia.org/wiki/Farey_sequence (Farey neighbours) and
    doesn't use njit to see how the two compare when you scale up this
    calculation.

    :param max_denominator: maximum denominator value
    :type max_denominator: int
    :param target_value: value to look left of, defaults to 3/7
    :type target_value: float, optional
    :return: numerator of left-most value
    :rtype: int
    """
    first_term = np.array([0, 1])
    second_term = np.array([1, 1])
    while True:
        combined_term = first_term + second_term
        fraction_combined_term = combined_term[0] / combined_term[1]
        if combined_term[1] > max_denominator:
            break

        if fraction_combined_term >= target_value:
            second_term = combined_term
        elif fraction_combined_term < target_value:
            first_term = combined_term

    return first_term[0]


if __name__ == "__main__":
    # start_time = time()
    # print("The answer is: {}".format(farey_sequence_normal(100_000_000)))
    # elapsed_time = time() - start_time
    # print("The answer was found in {0:.4f} s.".format(elapsed_time))

    compilation_start_time = time()
    farey_sequence_jitted(8)
    compilation_elapsed_time = time() - compilation_start_time
    print("Compile Time: {0:.4f} s.".format(compilation_elapsed_time))

    start_time = time()
    print("The answer is: {}".format(farey_sequence_jitted(1_000_000)))
    elapsed_time = time() - start_time
    print("The answer was found in {0:.4f} s.".format(elapsed_time))

    # start_time = time()
    # print("The answer is: {}".format(farey_sequence_jitted(100_000_000)))
    # elapsed_time = time() - start_time
    # print("The answer was found in {0:.4f} s.".format(elapsed_time))
