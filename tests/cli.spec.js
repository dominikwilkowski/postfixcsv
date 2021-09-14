/***************************************************************************************************************************************************************
 *
 * cli.js unit tests
 *
 * @file - src/cli.js
 *
 * Tested methods:
 * ParseArgs
 *
 **************************************************************************************************************************************************************/


const path = require('path');

const { ParseArgs, ReadFile } = require('../src/cli.js');


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// ParseArgs
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
test('ParseArgs - Should return nothing if no files have been passed in', () => {
	expect( ParseArgs([ 'path/to/node', 'path/to/script' ]) )
		.toMatchObject({ file: undefined, separator: undefined });
});

test('ParseArgs - Should find the file', () => {
	expect( ParseArgs([ 'path/to/node', 'path/to/script', 'path/to/file' ]) )
		.toMatchObject({ file: 'path/to/file', separator: undefined });
});

test('ParseArgs - Should find the file and the separator', () => {
	expect( ParseArgs([ 'path/to/node', 'path/to/script', 'path/to/file', '-s', ';' ]) )
		.toMatchObject({ file: 'path/to/file', separator: ';' });
});

test(`ParseArgs - Should use it's own process.argv`, () => {
	expect( ParseArgs() )
		.toMatchObject({});
});


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// ReadFile
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
test('ReadFile - Should return the content of a file', () => {
	return ReadFile(path.normalize(`${__dirname}/mock/test.csv`)).then((content) => {
		expect(content).toBe(`b1 b2 +,2 b2 3 * -, ,+
a1     ,5         , ,7 2 /
c2 3 * ,1 2       , ,5 1 2 + 4 * + 3 -
`);
	});
});

test('ReadFile - Should error when file not found', () => {
	console.error = jest.fn();
	return ReadFile('./mock/test.csv').catch((error) => {
		expect(console.error.mock.calls[0][0]).toContain('./mock/test.csv');
		expect(error.code).toBe('ENOENT');
	});
});
