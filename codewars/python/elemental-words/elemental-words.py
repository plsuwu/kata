#!/usr/bin/env python

from elements import ELEMENTS
from itertools import islice


def walk(word, elements, iterated):
    letters = [x for x in word.lower()]
    # letter_iter = iter(letters)

    if "".join(elements).lower() == word:
        return elements

    for window_size in range(len(letters)):
        for array_index in range(len(letters) - window_size):
            slice = letters[
                array_index : len(letters) - window_size
            ]
            if ''.join(slice) != '':

                for idx, el in enumerate(sorted(ELEMENTS.keys())):
                    if el not in iterated and el.lower() == "".join(slice):

                        print(f"current slice: '{slice}'\n  -> idx: {array_index}, window: {len(letters) - window_size}.")
                        iterated.append([el, ELEMENTS[el]])
                        elements.append(el)
                        print(f"{el=}")
                        for p in range(len([e for e in el])):
                            slice.pop(p - 1)
                            print(slice)

    # print(iterated)
    # print(elements)

    return elements


def elemental_forms(word):
    elements = []
    iterated = []

    walk(word, elements, iterated)


test_case = "Yes"
print(elemental_forms(test_case))

# test_case = "snack"
# print(elemental_forms(test_case))
