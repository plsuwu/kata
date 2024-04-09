#!/usr/bin/env python

import codewars_test as test

def decompose(n: int) -> list[int]:
    print(n ** 2)
    print('sqrt -> ', n**0.5)





decompose(5)
print(f"3^2 ({3**2}) + 4^2 ({4**2}) = ", (3 ** 2) + (4 ** 2), '\n')

decompose(11)
print(f"1^2 ({1**2}) + 2^2 ({2**2}) + 4^2 ({4**2}) + 10^2 ({10**2}) = {(1**2) + (2**2) + (4**2) + (10**2)}\n")

decompose(50)
print(f"1^2 ({1**2}) + 3^2 ({3**2}) + 5^2 ({5**2}) + 8^2 ({8**2}) + 49^2 ({49**2}) = {(1**2) + (3**2) + (5**2) + (8**2) + (49**2)}")

# @test.describe("Sample tests")
# def sample_tests():
#     @test.it("Static tests")
#     def static_tests():
#         test.assert_equals(decompose(5), [3,4])
#         test.assert_equals(decompose(8), None)

