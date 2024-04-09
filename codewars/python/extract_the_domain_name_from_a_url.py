#!/usr/bin/env python

def domain_name(url):

    string_opts = ["http://www.", "https://www.", "www.", "http://", "https://"]

    for o in string_opts:
        if o in url:
            s = url.split(o)
            return s[-1].split(".")[0]

    return url.split(".")[0]

tests = [
    "http://google.com",
    "http://google.co.jp",
    "https://123.net",
    "https://hyphen-site.org",
    "www.xakep.ru",
    "icann.org",
    "http://www.codewars.com/kata/"
]

for t in tests:
    test = domain_name(t)
    print(test)
