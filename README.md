POSTFIX-CSV
===========


## The task

- Create a cli node app that parses a csv spreadsheet file and outputs it's content
- The spreadsheet should be able to support:
	- [Reverse Polish Notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) calculations
		- Implement only the four most common operators: `+`,`-`,`/` and `*`
	- References to other cells via letter number notation (`A1` or `Z987`)
		- Letters refer to columns, numbers to rows
- When errors are found the output for that cell should be `#ERR`
- You're not allowed to use any packages from npm
- The app should print the output to `stdout`

The spreadsheet format coordination system should be `[Col (alphabetical)][Row (numeric)]`

```
| A1 | B1 | C1 |
----------------
| A2 | B2 | C2 |
----------------
| A3 | B3 | C3 |
```


Test file:

```csv
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

Think about edge cases and deal with them.


## What we are looking for

The task specifically does not mention:
- How to deal with whitespace in the csv
- What the format of the csv is (comma separated or otherwise)
- How to deal with when a cell is empty
- How to deal with more rows in a csv than the English alphabet
- How to deal with an infinite reference loop
- How the cli interface is defined


## What I did

Download the repo and run `npm link` inside the folder.
Now run the package against your csv file:

```shell
$ postfixcsv tests/mock/test.csv
```

If you got an CSV file that is semicolon separated or uses any other separator you can use the custom flag to parse it correctly.

```shell
$ postfixcsv tests/mock/test.csv -s \;
```

_(Semicolons have to be escaped in most shells)_


## Test

To run the unit tests make sure you `npm i` and run `npm test` in the root folder.


## Report

It took pretty much 3h 20min with a good hour of wife distraction right in the middle. I decided to add unit tests in the end which added another 45min. All
that time includes me reminding myself what the polish reverse notation was again :)


### Code structure

I used as many pure functions as I could in the short time for testability and composure.
The entry point for the package is `src/postfixcsv.js` while the `bin` command runs through `./index.js`. This is so that you may use the pure API and abstract
the CLI use on top.

`src/postfixcsv.js` does the heavy lifting while `src/cli.js` only does the cli work that is not relevant for the core code.
`./index.js` then just ties them both together and executes.


### Limitations

- The package will produce bugs with spreadsheets that have more than 701 columns. I did no feel I should add a catch there for the purpose of this small test.
  (For the sake of staying close to what spreadsheet apps do otherwise I would have simply implemented an numerical postfix which does not have any limitations)
- The data is currently looped over twice for creating a GRID object and the actual render loop. This can and should be optimized in the future.
- I would also look into a more battle tested `IsNumber` method as mine seems to work but I'm not sure in what condition it may fail ;)


### Design decisions

- I felt it was important to add dependency loop detection in as it would be incredible easy to run into an infinite loop when writing the CSV.
- I also added a custom flag for the CLI and custom option for the API to overwrite the separator for the CSV as there are so many different flavors out there.
- The error handler and CLI output was kept to an absolute minimum for time reasons. For production code I would add more love here
  (namely man page, help, color output etc)
- I added unit tests via [Jest](https://facebook.github.io/jest/) as I wasn’t sure what `thoroughly test your application` meant. For real coverage and more
  meaningful tests I would add an [end-to-end test](https://github.com/cuttlebelle/cuttlebelle/blob/master/tests/tester.js) like the one I wrote for
  Cuttlebelle where I run the CLI or API and compare the output against a bunch of fixtures in addition to unit tests.
