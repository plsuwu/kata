#!/usr/bin/env python

import codewars_test as test

class Node:
    def __init__(self, key):
        self.key = key
        self.l = None
        self.r = None

def radix_tree(*words: str):
    wordlist = [*words]
    struct: list[int] = []
    tree: dict[str,str] = {}

    for w_i, word in enumerate(wordlist):
        if w_i + 1 < len(wordlist):

            print(f"trying index '{w_i}' ({word}) & '{w_i + 1}' ({wordlist[w_i+1]}):")

            for l_i, letter in enumerate(word):
                print(l_i, letter)


# radix_tree("ape", "apple")
radix_tree("ape", "appendix", "apel")
# radix_tree("romane", "romanus", "romulus", "rubens", "rubicon", "rubicundus")

