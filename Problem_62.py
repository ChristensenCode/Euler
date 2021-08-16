"""
The cube, 41063625 (3453), can be permuted to produce two other cubes: 56623104 (3843) and 66430125 (4053).
In fact, 41063625 is the smallest cube which has exactly three permutations of its digits which are also cube.
Find the smallest cube for which exactly five permutations of its digits are cube.
"""

from time import time
import numpy as np
import pandas as pd
from numba import njit, prange, vectorize
from itertools import permutations
from pprint import pprint


def convert_to_integer(perms):
    output = []
    for i in perms:
        output.append(np.int(''.join(i)))
    return output


def cube_root_cleaner(modified_perm):
    keepers = []
    for x in modified_perm:
        if str(np.cbrt(x)).split('.')[1] == '0':
            keepers.append(x)
    return keepers


possibles = list(permutations('41063625'))
change_to_integer = [int(''.join(i)) for i in possibles]
cube_root = map(lambda x: x if str(np.cbrt(x)).split('.')[1] == '0' else None, change_to_integer)
cube_root_df = pd.Series(cube_root).dropna()
pprint(pd.unique(cube_root_df))
# cube_root = cube_root_array[np.logical_not(np.isnan(cube_root_array))]
integers = convert_to_integer(possibles)
print(cube_root_cleaner(integers))
