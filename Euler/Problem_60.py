from itertools import product, permutations, combinations


import numpy as np
import pandas as pd
from time import time
from pprint import pprint
import pyglet


# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# The primes 3, 7, 109, and 673, are quite remarkable.
# By taking any two primes and concatenating them in any order the result will always be prime.
# For example, taking 7 and 109, both 7109 and 1097 are prime.
# The sum of these four primes, 792, represents the lowest sum for a set of four primes with this property.
#
# Find the lowest sum for a set of five primes for which any two primes concatenate to produce another prime.
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

start_time = time()


def primesfrom2to(n):
    """ Input n>=6, Returns a array of primes, 2 <= p < n """
    sieve = np.ones(n//3 + (n%6==2), dtype=np.bool)
    for i in range(1,int(n**0.5)//3+1):
        if sieve[i]:
            k=3*i+1|1
            sieve[       k*k//3     ::2*k] = False
            sieve[k*(k-2*(i&1)+4)//3::2*k] = False
    return np.r_[2, 3, ((3*np.nonzero(sieve)[0][1:]+1)|1)]

total_primes = primesfrom2to(100)
total_primes_string = list(np.char.mod('%d', total_primes))

primes_check = primesfrom2to(10000000)
primes_check_string = set(np.char.mod('%d', primes_check))

test_list = [3, 7, 109, 673]
# df = pd.read_csv('../data/primes1.txt', header=None, delim_whitespace=True)
# print(df)
# print(len(df.columns))
# for i in range(len(df)):
#     for j in range(len(df.columns)):
#         pass

answer_array = []
for i in total_primes_string:
    answer_list = []
    for j in total_primes_string:
        if i + j in primes_check_string and j + i in primes_check_string:
            answer_list.append(j)

    answer_list.append(i)
    answer_list = answer_list[-1:] + answer_list[:-1]
    if len(answer_list) > 1:
        answer_array.append(answer_list)
pprint(answer_array)

answer_array_2 = []
for first_pass in answer_array:
    print(first_pass)
    answer_list_2 = []
    for i in first_pass:
        for j in first_pass:
            if i + j in primes_check_string and j + i in primes_check_string:
                answer_list_2.append(j)
    # answer_list_2 = answer_list_2[-1:] + answer_list_2[:-1]
    if len(answer_list_2) > 1:
        answer_array_2.append(answer_list_2)

pprint(answer_array_2)
# THIS IS NEW
# comment from laptop







# answer_array_3 = []
# for first_pass in answer_array_2:
#     for i in first_pass:
#         answer_list_3 = []
#         for j in first_pass:
#             if i + j in primes_check_string and j + i in primes_check_string:
#                 answer_list_3.append(j)
#
#     answer_list_3.append(i)
#     if len(answer_list_3) > 1:
#         answer_array_3.append(sorted(list(set(answer_list_3))))

# pprint(answer_array_3)

# answer_array_4 = []
# for first_pass in answer_array_3:
#     for i in first_pass:
#         answer_list_4 = []
#         for j in first_pass:
#             if i + j in primes_check_string and j + i in primes_check_string:
#                 answer_list_4.append(j)
#
#     answer_list_4.append(i)
#     if len(answer_list_4) > 1:
#         answer_array_4.append(sorted(list(set(answer_list_4))))

# pprint(answer_array_4)

# answer_array_5 = []
# for first_pass in answer_array_4:
#     for i in first_pass:
#         answer_list_5 = []
#         for j in first_pass:
#             if i + j in primes_check_string and j + i in primes_check_string:
#                 answer_list_5.append(j)
#
#     answer_list_5.append(i)
#     if len(answer_list_5) > 1:
#         answer_array_5.append(sorted(list(set(answer_list_5))))
# pprint(answer_array_5)


elapsed_time = time() - start_time
print('The answer was found in {0:.4f} s.'.format(elapsed_time))
