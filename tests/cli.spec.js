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


const ParseArgs = require('../src/cli.js');


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
