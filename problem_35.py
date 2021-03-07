import numpy as np
from time import time
from pprint import pprint

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# How many circular primes are there below one million?
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

start_time = time()


def prime_hunter(length):
    primes = []
    for x in range(2, length + 1):
        isPrime = True
        for y in range(2, int(x ** 0.5) + 1):
            if x % y == 0:
                isPrime = False
                break
        if isPrime:
            primes.append(x)
    return primes
counter = 0
possible_primes = prime_hunter(1000000)
rotation_primes = []
prime_set = set(possible_primes)

for value in possible_primes:
    value = str(value)
    value_length = len(value)
    rotated_value = value[value_length - 1:] + value[:value_length - 1]
    possible_solution = []
    for N in rotated_value:
        rotated_value = rotated_value[value_length - 1:] + rotated_value[:value_length - 1]
        possible_solution.append(int(rotated_value))
    possible_set = set(possible_solution)
    if possible_set.issubset(prime_set):
        rotation_primes.append(value)
        counter += 1
elapsed_time =time() - start_time
print("Compute Time: " + str(elapsed_time) + " seconds")
print("Answer: " + str(counter))
