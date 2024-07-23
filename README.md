POSTFIX-CSV
===========


## The task

- Create a cli app that parses a csv file and outputs its content
- The spreadsheet should be able to support:
	- [Reverse Polish Notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) calculations
		- Implement only the four most common operators: `+`,`-`,`/` and `*`
	- References to other cells via letter number notation (`A1` or `Z987`)
		- Letters refer to columns, numbers to rows
		- The coordinates start with 1 not zero
- When errors are found, the output for that cell should be `#ERR`
- You're not allowed to use any external packages (usage of standard library is allowed)
- Print the output to `stdout`

The spreadsheet coordination system should be `[Col (alphabetical)][Row (numeric)]`

```
| A1 | B1 | C1 |
----------------
| A2 | B2 | C2 |
----------------
| A3 | B3 | C3 |
```

Test csv file:

```
B1 B2 +,2 B2 3 * -,+
A1,5,7 2 /
C2 3 *,1 2,5 1 2 + 4 * + 3 -
```

Which should output:

```
-8,-13,#ERR
-8,5,3.5
10.5,#ERR,14
```
