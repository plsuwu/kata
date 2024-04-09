#!/usr/bin/env python

# python is getting SO FUCKIGN confusing for this.........

# https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/

def min_remove(s: str) -> str:
    chars = [letter for letter in s]
    closed = chars.count("(") - chars.count(")")

    p_index: dict[str, list[int]] = {
            "(": [],
            ")": []
    }

    # find paren positions
    for i, c in enumerate(chars):
        if c != "(" and c != ")":
            continue

        if c == "(":
            print(f"'(' found @ {i}")
            p_index["("].insert(0, i)
            continue

        else:
            print(f"')' found @ {i}")
            p_index[")"].insert(0, i)
            continue

    print(p_index)

    if closed == 0 and chars.index("(") > chars.index(")"):
        chars.pop(p_index[")"][0])
        p_index[")"].pop(0)
        closed += 1

    while closed != 0:
        if closed < 0:
            # paren sequence doesn't start with opener
            if p_index["("][0] < p_index[")"][0]:
                print("parens dont begin with opener!")
                chars.pop(p_index[")"][-1])
                p_index[")"].pop(-1)

            else:
                chars.pop(p_index[")"][0])
                p_index[")"].pop(0)

            closed += 1

        else:
            chars.pop(p_index["("][0])
            p_index["("].pop(0)
            closed -= 1


    print("".join(chars))

    return s


def main():
    cases = {"one": ["lee(t(c)o)de)", "lee(t(c)o)de"], "two": ["a)b(c)d", "ab(c)d"], "three": ["a(b)c(d", "a(b)cd"], "four": [")(abc)(d", "(abc)d"] }

    for k, v in cases.items():
        print(f"\ntest case {k}: input -> {v[0]}, expecting {v[1]}")
        min_remove(v[0])
        # real = min_remove(v[0])
        # print(f"actual func output: {real}")
        print("\n")


if __name__ == "__main__":
    main()
