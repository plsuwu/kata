#!/usr/bin/env python

import codewars_test as test


def kprimes_step(k: int, step: int, start: int, nd: int) -> list[int]:
    """
    @params:
        - k: int > 0
        - step: int > 0
        - start: int >= 0
        - nd: int >= start

    @return:
        - list[list?[list??[int, int??], [int?, int??], ?? ]]
        ex: kprimes_step(2, 2, 0, 50) -> [[4, 6], [33, 35]]
    """
    print(k, step, start, nd)


@test.describe("Fixed Tests")
def fixed_tests():
    @test.it("Basic Test Cases")
    def basic_test_cases():
        test.assert_equals(kprimes_step(10, 8, 2425364, 2425700), [])
        test.assert_equals(
            kprimes_step(6, 8, 2627371, 2627581),
            [[2627408, 2627416], [2627440, 2627448], [2627496, 2627504]],
        )
