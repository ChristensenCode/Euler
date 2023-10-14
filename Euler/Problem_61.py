import numpy as np
from time import time
from pprint import pprint
import pandas as pd

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# Triangle, square, pentagonal, hexagonal, heptagonal, and octagonal numbers
# are all figurate (polygonal) numbers and are generated by the following formulae:
# Triangle	 	    P3,n=n(n+1)/2	 	1, 3, 6, 10, 15, ...
# Square	 	    P4,n=n2	 	        1, 4, 9, 16, 25, ...
# Pentagonal	 	P5,n=n(3n−1)/2	 	1, 5, 12, 22, 35, ...
# Hexagonal	    	P6,n=n(2n−1)	 	1, 6, 15, 28, 45, ...
# Heptagonal	 	P7,n=n(5n−3)/2	 	1, 7, 18, 34, 55, ...
# Octagonal	 	    P8,n=n(3n−2)	 	1, 8, 21, 40, 65, ...
# The ordered set of three 4-digit numbers: 8128, 2882, 8281, has three interesting properties.
#
# The set is cyclic, in that the last two digits of each number is the first two digits of the
# next number (including the last number with the first).
# Each polygonal type: triangle (P3,127=8128), square (P4,91=8281), and pentagonal (P5,44=2882),
# is represented by a different number in the set.
# This is the only set of 4-digit numbers with this property.
# Find the sum of the only ordered set of six cyclic 4-digit numbers
# for which each polygonal type: triangle, square, pentagonal, hexagonal, heptagonal, and octagonal,
# is represented by a different number in the set.
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

"""
What do we know?
1. 4 digit number 1000 - 9999
2. cyclical
3. polygon
"""

start_time = time()


def tri(values):
    return values * (values + 1)/2


def square(values):
    return values ** 2


def penta(values):
    return values * (3 * values - 1)/2


def hex(values):
    return values * (2 * values - 1)


def hepta(values):
    return values * (5 * values - 3)/2


def octo(values):
    return values * (3 * values - 2)


figurate_range = np.arange(20, 140)
figurate_db = pd.DataFrame()
figurate_db['tri'] = tri(figurate_range)
figurate_db['square'] = square(figurate_range)
figurate_db['penta'] = penta(figurate_range)
figurate_db['hex'] = hex(figurate_range)
figurate_db['hepta'] = hepta(figurate_range)
figurate_db['octo'] = octo(figurate_range)
df = figurate_db.astype('int64')
df = df[(df > 1000) & (df < 10000)]
df = df.astype('str')
possible_solutions = np.arange(1000, 1100)
possible_solutions = possible_solutions.astype('str')

new_endings = np.arange(11, 100).astype('str')
print(df.to_string())








elapsed_time = time() - start_time
print('The answer was found in {0:.4f} s.'.format(elapsed_time))
