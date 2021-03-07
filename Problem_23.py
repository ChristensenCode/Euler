# Problem 23
# Non-Abundant sums

# Creates a list of abundant numbers
from pprint import pprint

def proper_divisor(n):
    divisors = []
    for i in range(1,n):
        if n % i == 0:
            divisors.append(i)

    if sum(divisors) > n:
        return n
abundant_numbers = []
n = 0
combined_values = []
for x in range(1,28124):
    if proper_divisor(x) is not None:
        abundant_numbers.append(x)
        #print('-'*60)
        #print(abundant_numbers[n])
        #print('-'*60)
        for value in abundant_numbers:
            combined = abundant_numbers[n]+value
            #print(combined)
            combined_values.append(combined)
        n += 1
#pprint(set(abundant_numbers))
numbers = []
for i in range(1,28124):
    numbers.append(i)
number_set = set(numbers)
combined_numbers = set(combined_values)
cannot = number_set.difference(combined_numbers)
print(sum(cannot))
