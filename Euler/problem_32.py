from time import time

#==================================================================
# Find the sum of all products whose multiplicand/multiplier/product
# identity can be written as a 1 through 9 pandigital.
#==================================================================

start_time = time()
summations = []
pandigital = ['1', '2', '3', '4', '5', '6', '7', '8', '9']

for i in range(2001):
    string_i = str(i)
    if '0' in string_i:
        continue
    else:
        for j in range(2001):
            string_j = str(j)
            if '0' in string_j:
                continue
            else:
                product = i * j
                string_product = str(product)
                if '0' in string_product:
                    continue
                else:
                    value = str(product) + str(i) + str(j)
                    integer_value = int(value)
                    sorted_integer_value = sorted(value)
                    if sorted_integer_value == pandigital:
                        summations.append(product)

set_summations = set(summations)
elapsed_time = time() - start_time
print(elapsed_time)
print(sum(set_summations))
