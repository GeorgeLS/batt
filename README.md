## Batt

Batt stands for boolean algebra truth table.
This program takes as input an arbitrary boolean algebra expression and prints the truth table for that expression.

i.e

Input:

```
A && B
```

Output:

```
------------
|A|B|A && B|
------------
|0|0|     0|
------------
|0|1|     0|
------------
|1|0|     0|
------------
|1|1|     1|
------------
```

This program will be extended in the future so as to check if the given expression can evaluate to TRUE (1) for any given combination of variable inputs.
It will also be extended to check if two given inputs are equivalent in terms of output.
Finally I would like to make this program multi threaded as it is embarrassingly parallel to compute the evaluation for a given input.
