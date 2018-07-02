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


const {
	INDEXES,
	CleanCSV,
	GetCol,
	MakeGrid,
	IsCoordinate,
	IsOperator,
	IsNumber,
	ParsePostfix,
	ParseCells,
	Postfixcsv,
} = require('../src/postfixcsv.js');


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
		.toBe('ZZ');
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

	expect( IsNumber(20000.1) )
		.toBe( true );

	expect( IsNumber('-324.8') )
		.toBe( true );

	expect( IsNumber(-324.8) )
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
	expect( ParsePostfix( '', [], 'A1' ) )
		.toMatchObject({
			errors: [],
			expression: 0,
		});

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
				"ERROR: Expression at >>A1<< not valid (1)",
			],
			expression: '#ERR',
		});

	expect( ParsePostfix( '2 B2 +', [{ A: '2 B2 +' }, { B: '2 A1 +' }, { B: '3 2 B1 * +' }], 'A1' ) )
		.toMatchObject({
			errors: [
				"ERROR: Dependency loop detected in A1 -> B2",
				"ERROR: Expression at >>B2<< not valid (1)",
				"ERROR: Expression at >>A1<< not valid (1)",
			],
			expression: '#ERR',
		});
});

test('ParsePostfix - Should not double up on errors for same field', () => {
	expect( ParsePostfix( '1 + 1', [], 'A1' ) )
		.toMatchObject({
			errors: [
				"ERROR: Expression at >>A1<< not valid (1)",
			],
			expression: '#ERR',
		});

	expect( ParsePostfix( '1 1 + + 1 +', [], 'A1' ) )
		.toMatchObject({
			errors: [
				"ERROR: Expression at >>A1<< not valid (1)",
			],
			expression: '#ERR',
		});
});

test('ParsePostfix - Both error messages are added correctly', () => {
	expect( ParsePostfix( '+', [], 'A1' ) )
		.toMatchObject({
			errors: [
				"ERROR: Expression at >>A1<< not valid (1)",
			],
			expression: '#ERR',
		});

	expect( ParsePostfix( '1 2', [], 'A1' ) )
		.toMatchObject({
			errors: [
				"ERROR: Expression at >>A1<< not valid (2)",
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

	expect( ParseCells( '1 2 +', [], ',' ) )
		.toMatchObject({
			errors: [],
			output: '3',
		});
});


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Postfixcsv
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
test('Postfixcsv - Should parse the CSV content correctly', () => {
	expect( Postfixcsv( '1 2 +, 2 2 -\n1 1 *,2 3 5  * +\n1 + 2,A1  B1 +' ) )
		.toMatchObject({
			errors: [
				'ERROR: Expression at >>A3<< not valid (1)',
			],
			output: '3,0\n1,17\n#ERR,3',
		});
});
