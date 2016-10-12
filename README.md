# email-rs Email body parser

RFC2822-compliant email parser, built with chomp


## Benchmarks

```
running 18 tests
test example_1_1_1            ... bench:         659 ns/iter (+/- 326)
test example_1_1_1_date       ... bench:       4,535 ns/iter (+/- 773)
test example_1_1_1_from       ... bench:      15,659 ns/iter (+/- 4,440)
test example_1_1_1_message_id ... bench:       3,529 ns/iter (+/- 943)
test example_1_1_1_subject    ... bench:         111 ns/iter (+/- 38)
test example_1_1_2            ... bench:         780 ns/iter (+/- 415)
test example_1_2              ... bench:         802 ns/iter (+/- 145)
test example_1_3              ... bench:         691 ns/iter (+/- 182)
test example_2_1              ... bench:         670 ns/iter (+/- 171)
test example_2_2              ... bench:       1,024 ns/iter (+/- 481)
test example_2_3              ... bench:       1,033 ns/iter (+/- 636)
test example_3_1              ... bench:         711 ns/iter (+/- 147)
test example_3_2              ... bench:       1,346 ns/iter (+/- 287)
test example_4                ... bench:         967 ns/iter (+/- 441)
test example_5                ... bench:       1,050 ns/iter (+/- 415)
test example_6_1              ... bench:         582 ns/iter (+/- 189)
test example_6_2              ... bench:         679 ns/iter (+/- 343)
test example_6_3              ... bench:         299 ns/iter (+/- 92)
```
