/***************************************************************************************************************************************************************
 *
 * postfixcsv.js unit tests
 *
 * @file - src/postfixcsv.js
 *
 * Tested methods:
 * INDEXES
 * CleanCSV
 * GetCol
 * MakeGrid
 * IsCoordinate
 * IsOperator
 * IsNumber
 * ParsePostfix
 * ParseCells
 * Postfixcsv
 *
 **************************************************************************************************************************************************************/


const PostfixcsvExport = require('../src/postfixcsv.js');

const INDEXES = PostfixcsvExport.INDEXES;
const CleanCSV = PostfixcsvExport.CleanCSV;
const GetCol = PostfixcsvExport.GetCol;
const MakeGrid = PostfixcsvExport.MakeGrid;
const IsCoordinate = PostfixcsvExport.IsCoordinate;
const IsOperator = PostfixcsvExport.IsOperator;
const IsNumber = PostfixcsvExport.IsNumber;
const ParsePostfix = PostfixcsvExport.ParsePostfix;
const ParseCells = PostfixcsvExport.ParseCells;
const Postfixcsv = PostfixcsvExport.Postfixcsv;


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// INDEXES
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
test('INDEXES - Should be an object', () => {
	expect( typeof INDEXES )
		.toBe('object');
});

test('INDEXES - Should have strings inside', () => {
	expect( typeof INDEXES[ 0 ] )
		.toBe('string');
});


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// CleanCSV
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
test('CleanCSV - Should clean a string properly', () => {
	expect( CleanCSV('test\ntest\r\n  spaces     spaces') )
		.toBe('test\ntest\n spaces spaces');
});


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// GetCol
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
test('GetCol - Should give us the right column index', () => {
	expect( GetCol( 0 ) )
		.toBe('A');

	expect( GetCol( 10 ) )
		.toBe('K');

	expect( GetCol( 25 ) )
		.toBe('Z');

	expect( GetCol( 26 ) )
		.toBe('AA');

	expect( GetCol( 701 ) )
		.toBe('SG');
});


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// MakeGrid
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
test('MakeGrid - Should make a nice object from a string', () => {
	expect( MakeGrid('A1,B1,C1\nA2,B2,C2\nA3,B3,C3', ',') )
		.toMatchObject([
			{"A": "A1", "B": "B1", "C": "C1"},
			{"A": "A2", "B": "B2", "C": "C2"},
			{"A": "A3", "B": "B3", "C": "C3"},
		]);

	expect( MakeGrid(',,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,Hello', ',') )
		.toMatchObject([{
			"A": "",
			"B": "",
			"C": "",
			"D": "",
			"E": "",
			"F": "",
			"G": "",
			"H": "",
			"I": "",
			"J": "",
			"K": "",
			"L": "",
			"M": "",
			"N": "",
			"O": "",
			"P": "",
			"Q": "",
			"R": "",
			"S": "",
			"T": "",
			"U": "",
			"V": "",
			"W": "",
			"X": "",
			"Y": "",
			"Z": "",
			"AA": "",
			"AB": "",
			"AC": "",
			"AD": "",
			"AE": "",
			"AF": "",
			"AG": "",
			"AH": "",
			"AI": "Hello",
		}]);
});


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// IsCoordinate
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
test('IsCoordinate - Should detect coordinates', () => {
	expect( IsCoordinate('D2') )
		.toBe( true );

	expect( IsCoordinate('2D') )
		.toBe( false );

	expect( IsCoordinate('AAAD2028') )
		.toBe( true );

	expect( IsCoordinate('C3PO') )
		.toBe( false );

	expect( IsCoordinate('2') )
		.toBe( false );

	expect( IsCoordinate('A') )
		.toBe( false );
});


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// IsOperator
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
test('IsOperator - Should detect operator', () => {
	expect( IsOperator('+') )
		.toBe( true );

	expect( IsOperator('-') )
		.toBe( true );

	expect( IsOperator('*') )
		.toBe( true );

	expect( IsOperator('/') )
		.toBe( true );

	expect( IsOperator('#') )
		.toBe( false );

	expect( IsOperator('%') )
		.toBe( false );

	expect( IsOperator('') )
		.toBe( false );

	expect( IsOperator('000') )
		.toBe( false );

	expect( IsOperator( undefined ) )
		.toBe( false );
});


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// IsNumber
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
test('IsNumber - Should detect numerical numbers', () => {
	expect( IsNumber('1') )
		.toBe( true );

	expect( IsNumber(1) )
		.toBe( true );

	expect( IsNumber('20000.1') )
		.toBe( true );

	expect( IsNumber('-324.8') )
		.toBe( true );

	expect( IsNumber('E534') )
		.toBe( false );

	expect( IsNumber('D2') )
		.toBe( false );

	expect( IsNumber('2D') )
		.toBe( false );

	expect( IsNumber('D2') )
		.toBe( false );
});


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// ParsePostfix
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
test('ParsePostfix - Should calculate reverse polish notation correctly', () => {
	expect( ParsePostfix( '2 1 +', [], 'A1' ) )
		.toMatchObject({
			errors: [],
			expression: 3,
		});

	expect( ParsePostfix( '2 5 5 * +', [], 'A1' ) )
		.toMatchObject({
			errors: [],
			expression: 27,
		});

	expect( ParsePostfix( '2 1 - 3 5 * + 10 /', [], 'A1' ) )
		.toMatchObject({
			errors: [],
			expression: 1.6,
		});
});

test('ParsePostfix - Should be able to handle coordinates', () => {
	expect( ParsePostfix( 'A1 A2 +', [{ A: '10' }, { A: '20' }], 'D5' ) )
		.toMatchObject({
			errors: [],
			expression: 30,
		});
});

test('ParsePostfix - Should be able to handle dependency loop', () => {
	expect( ParsePostfix( 'A1 A2 +', [{ A: '10' }, { A: '20' }], 'A1' ) )
		.toMatchObject({
			errors: [
				"ERROR: Dependency loop detected in A1",
				"ERROR: Expression at >>A1<< no valid (1)",
			],
			expression: '#ERR',
		});
});


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// ParseCells
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
test('ParseCells - Should parse several cells in a row', () => {
	expect( ParseCells( '1 2 +, 2 2 *', [], ',' ) )
		.toMatchObject({
			errors: [],
			output: '3,4',
		});
});


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Postfixcsv
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
test('Postfixcsv - Should parse a file correctly', () => {
	return Postfixcsv({ file: 'tests/mock/test.csv' }).then( data => {
		expect( data ).toMatchObject({
			errors: [
				"ERROR: Expression at >>D1<< no valid (1)",
				"ERROR: Expression at >>B3<< no valid (2)",
			],
			output: '-8,-13,0,#ERR\n-8,5,0,3.5\n0,#ERR,0,14',
		});
	});
});
