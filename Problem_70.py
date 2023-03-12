"""
Find the value of n, 1 < n < 107, for which φ(n) is a permutation 
of n and the ratio n/φ(n) produces a minimum.

Find the value of n ≤ 1,000,000 for which n/φ(n) is a maximum.
https://en.wikipedia.org/wiki/Euler%27s_totient_function
"""
from typing import List
from numba import njit
from time import time
from tqdm import tqdm
from sympy.ntheory import factorint, primefactors
import numpy as np
from math import sqrt
from euler_common import primesfrom2to

def fast_version():
    """
    https://blog.dreamshire.com/project-euler-70-solution/
    """
    # Limit defined in the problem statement
    limit = 10_000_000

    # Not sure why they square rooted and multiplied by 1.2
    # The totient has multiplicative properties
    # so if you take two relatively prime numbers and multiply
    # them together, their totients can also multiply.
    # We know that all primes are also coprime.
    primes = list(primesfrom2to(int(sqrt(limit))))
    
    # Not sure why you can remove even more primes.
    del primes[:int(0.6*len(primes))]
    min_q = 2
    min_n = 0
    for i, p1 in enumerate(primes):
        for p2 in primes[i:]:
            if (p1 + p2) % 9 != 1:
                continue
            n = p1 * p2
            if n > limit:
                return min_n
            phi = (p1-1) * (p2-1)
            q = n / phi
            if sorted(str(phi)) == sorted(str(n)) and min_q>q: 
                min_q, min_n = q, n

@njit(fastmath=True)
def mathy(base, factor):
    return base ** (factor - 1) * (base - 1)

def main():
    max_iteration = 10_000_000 + 10
    min_ratio = max_iteration
    answer = 0
    for n in tqdm(range(10, max_iteration)):
        # prime_factors = np.array(primefactors(n))
        # totient = int(one_minus_inverted_prime_factors(n, prime_factors))
        totient = 1

        for base, factor in factorint(n).items():
            totient *= mathy(base, factor)

        ratio_test = n / totient
        if ratio_test >= min_ratio:
            continue

        if sorted(str(n)) == sorted(str(totient)):
            answer = n
            min_ratio = ratio_test
    return answer


if __name__ == "__main__":
    
    start_time = time()
    # print("The answer is: {}".format(main()))
    print("The answer is: {}".format(fast_version()))
    elapsed_time = time() - start_time
    print("The answer was found in {0:.4f} s.".format(elapsed_time))
