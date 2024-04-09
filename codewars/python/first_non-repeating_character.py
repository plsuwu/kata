#!/usr/bin/env python

import codewars_test as test


def first_non_repeating_letter(s: str) -> str:
    idx = [i for i, letter in enumerate(s.lower()) if s.lower().count(letter) == 1]

    return s[idx[0]] if len(idx) > 0 else ""


@test.describe("Sample Tests")
def basic_tests():
    @test.it("Should handle simple cases")
    def _():
        test.assert_equals(first_non_repeating_letter("a"), "a")
        test.assert_equals(first_non_repeating_letter("stress"), "t")
        test.assert_equals(first_non_repeating_letter("moonmen"), "e")

    @test.it("Should handle exotic characters")
    def _():
        test.assert_equals(first_non_repeating_letter("~><#~><"), "#")
        test.assert_equals(first_non_repeating_letter("hello world, eh?"), "w")

    @test.it("Should handle strings without unique characters")
    def _():
        test.assert_equals(first_non_repeating_letter("abba"), "")
        test.assert_equals(first_non_repeating_letter("aa"), "")
