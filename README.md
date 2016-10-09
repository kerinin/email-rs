# email-rs Email body parser

RFC2822-compliant email parser, built with chomp


## Benchmarks

```
running 14 tests
test example_1_1_1 ... bench:      45,033 ns/iter (+/- 19,515)
test example_1_1_2 ... bench:      66,765 ns/iter (+/- 24,096)
test example_1_2   ... bench:     103,040 ns/iter (+/- 43,135)
test example_1_3   ... bench:     131,014 ns/iter (+/- 58,848)
test example_2_1   ... bench:      56,952 ns/iter (+/- 19,580)
test example_2_2   ... bench:     107,388 ns/iter (+/- 35,390)
test example_2_3   ... bench:      91,489 ns/iter (+/- 36,536)
test example_3_1   ... bench:      52,056 ns/iter (+/- 57,030)
test example_3_2   ... bench:     100,962 ns/iter (+/- 32,393)
test example_4     ... bench:     218,572 ns/iter (+/- 117,183)
test example_5     ... bench:     258,097 ns/iter (+/- 98,811)
test example_6_1   ... bench:     123,389 ns/iter (+/- 52,117)
test example_6_2   ... bench:      58,943 ns/iter (+/- 23,048)
test example_6_3   ... bench:         396 ns/iter (+/- 1,889)
```

Just parsing 'Optional' fields

```
running 14 tests
test example_1_1_1 ... bench:      38,359 ns/iter (+/- 14,501)
test example_1_1_2 ... bench:      52,662 ns/iter (+/- 13,618)
test example_1_2   ... bench:      65,676 ns/iter (+/- 17,090)
test example_1_3   ... bench:      47,866 ns/iter (+/- 8,999)
test example_2_1   ... bench:      45,591 ns/iter (+/- 18,497)
test example_2_2   ... bench:      70,437 ns/iter (+/- 25,391)
test example_2_3   ... bench:      69,554 ns/iter (+/- 49,414)
test example_3_1   ... bench:      46,789 ns/iter (+/- 29,144)
test example_3_2   ... bench:      84,695 ns/iter (+/- 26,348)
test example_4     ... bench:     131,911 ns/iter (+/- 50,902)
test example_5     ... bench:     132,015 ns/iter (+/- 48,042)
test example_6_1   ... bench:      56,869 ns/iter (+/- 53,850)
test example_6_2   ... bench:      41,809 ns/iter (+/- 46,976)
test example_6_3   ... bench:          83 ns/iter (+/- 83)
```
