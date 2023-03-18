"""
By listing the set of reduced proper fractions for d ≤ 1,000,000 in 
ascending order of size, find the numerator of the fraction immediately to the left of 3/7.
"""

from math import gcd
from time import time
from pprint import pprint
from tqdm import tqdm
import numpy as np
from functools import partial

def primary_actions(input_number):
    sub_range = np.arange(1, input_number)
    gcd_values = np.fromiter(map(partial(gcd, input_number), sub_range), dtype='int32')
    locations_of_one = np.argwhere(gcd_values == 1)
    reduced_proper_fractions = (sub_range[locations_of_one] / input_number).flatten()
    reduced_numerators = sub_range[locations_of_one]
    return reduced_numerators, reduced_proper_fractions


def main():
    LIMIT = 1_000_000
    pretty_storage = {}
    for d in tqdm(range(1, LIMIT+1)):
        red_num, red_prop = primary_actions(d)
        for index, numerator in enumerate(red_num.flatten()):
            pretty_storage[f'{numerator}/{d}'] = red_prop[index]

    pretty_storage = sorted(pretty_storage.items(), key=lambda item: item[1])
    pprint(pretty_storage)
    x = 1

def fast_inner_loop(denominator_value: int, iteration_limit: int, value_to_compare: float = 3/7):
    for numerator in range(1, iteration_limit):
        decimal_representation = numerator / denominator_value
        if decimal_representation > value_to_compare:
            break
    return f"{numerator}/{denominator_value}", decimal_representation

def naive_solution():
    LIMIT = 1_000_000
    # LIMIT = 8

    LESS_THAN_LIMIT = 3 / 7

    simple_dict = {}
    for denominator in tqdm(range(1, LIMIT)):
        for numerator in range(1, LIMIT):
            decimal_representation = numerator / denominator
            if decimal_representation > LESS_THAN_LIMIT:
                break
            simple_dict[f'{numerator}/{denominator}'] = decimal_representation
    simple_dict = sorted(simple_dict.items(), key=lambda item: item[1])
    pprint(simple_dict)



if __name__ == "__main__":
    start_time = time()
    # print("The answer is: {}".format(main()))
    print("The answer is: {}".format(naive_solution()))
    elapsed_time = time() - start_time
    print("The answer was found in {0:.4f} s.".format(elapsed_time))