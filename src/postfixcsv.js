'use strict';


/**
 * An index collection of column descriptions
 *
 * @type {Array}
 */
const INDEXES = [ 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z' ];


/**
 * Clean CSV data input of messing CR and duplicate spaces
 *
 * @param  {string} CSV - The CSV file content
 *
 * @return {string}     - The cleaned CSV content
 */
const CleanCSV = ( CSV ) => CSV.replace( /\r?\n|\r/g, '\n' ).replace( /  +/g, ' ' );


/**
 * Get the Column coordinate for an array index
 *
 * @param  {integer} col - Array index
 *
 * @return {string}      - The column ID
 */
const GetCol = ( col ) => {
	const index = Math.floor( col / INDEXES.length );

	// making sure we can handle more than 26 cells by adding a postfix to each cell
	// (now supports 701 which is not infinite but at least aligns with what spreadsheet apps do)
	return `${ index > 0 ? INDEXES[ index - 1 ] : '' }${ INDEXES[ col - INDEXES.length * index ] }`;
};


/**
 * Make an object grid from a separated CSV string so we can navigate the cells
 *
 * @param  {string} data      - The data of a CSV file
 * @param  {string} separator - The separator of the CSV file
 *
 * @return {object}           - An object array
 */
const MakeGrid = ( data, separator ) => {
	const grid = [];

	data
		.split('\n')
		.forEach( line => {
			const cells = line.split( separator );
			const row = {};

			cells.forEach( ( cell, i ) => {
				const index = Math.floor( i / INDEXES.length );

				row[ GetCol( i ) ] = cell;
			});

			grid.push( row );
		});

	return grid;
};


/**
 * Test if an item is a coordinate
 *
 * @param  {string} item - The item to be tested
 *
 * @return {boolean}     - The result of the test
 */
const IsCoordinate = ( item ) => /^[a-zA-Z]+[0-9]+$/.test( item );


/**
 * Test if an item is an operator
 *
 * @param  {string} item - The item to be tested
 *
 * @return {boolean}     - The result of the test
 */
const IsOperator = ( item ) => [ '+', '-', '*', '/' ].includes( item );


/**
 * Test if an item is a number
 *
 * @param  {string|integer} item - The item to be tested
 *
 * @return {boolean}             - The result of the test
 */
const IsNumber = ( item ) => ( parseFloat( item ) - parseFloat( item ) + 1 ) === 1 && String( parseFloat( item ) ).length === String( item ).length;


/**
 * Parsing a postfix expression
 *
 * @param  {string} expression - The expression to be parsed
 * @param  {object} GRID       - The GRID object
 * @param  {object} thisCell   - The coordinate of this cell
 * @param  {object} parsed     - All previously parsed fields so we can detect a dependency loop
 *
 * @return {number}            - The result of the expression
 */
const ParsePostfix = ( expression, GRID, thisCell, parsed = [] ) => {
	if( expression === ' ' || expression === '' ) {
		expression = '0'; // I don’t agree, empty cells should be invalid...
	}

	const items = expression.split(' ');
	const queue = [];
	let errors = [];

	if( parsed.includes( thisCell ) ) {
		errors.push(`ERROR: Dependency loop detected in ${ parsed.join(' -> ') }`);

		return {
			expression: '#ERR',
			errors,
		};
	}

	items.forEach( item => {
		if( IsNumber( item ) ) {
			queue.push( parseFloat( item ) );
		}
		else if( IsCoordinate( item ) ) {
			const coordiantes = item.toUpperCase().match(/[a-zA-Z]+|[0-9]+/g);

			const thisTarget = GRID[ parseInt( coordiantes[ 1 ] - 1 ) ][ coordiantes[ 0 ] ];
			parsed.push( thisCell );
			const target = ParsePostfix( thisTarget, GRID, `${ coordiantes[ 0 ] }${ coordiantes[ 1 ] }`, parsed );

			errors = [ ...errors, ...target.errors ];

			queue.push( target.expression );
		}
		else if( IsOperator( item ) ) {
			const y = queue.pop();
			const x = queue.pop();
			let result;

			if( !IsNumber( x ) || !IsNumber( y ) ) {
				if( !errors.includes(`ERROR: Expression at >>${ thisCell }<< not valid (1)`) ) {
					errors.push(`ERROR: Expression at >>${ thisCell }<< not valid (1)`);
				}

				queue.push('#ERR');
			}
			else {
				if( item === '+' ) {
					result = x + y;
				}

				if( item === '-' ) {
					result = x - y;
				}

				if( item === '*' ) {
					result = x * y;
				}

				if( item === '/' ) {
					result = x / y;
				}

				queue.push( result );
			}
		}
	});

	if( queue.length > 1 ) {
		if( !errors.includes(`ERROR: Expression at >>${ thisCell }<< not valid (1)`) ) {
			errors.push(`ERROR: Expression at >>${ thisCell }<< not valid (2)`);
		}

		return {
			expression: '#ERR',
			errors,
		};
	}
	else {
		return {
			expression: queue[ 0 ],
			errors,
		};
	}

};


/**
 * Step through each cell and parse each expression
 *
 * @param  {string} CSV       - The CSV file contents cleaned
 * @param  {object} GRID      - The GRID object
 * @param  {string} separator - The separator of the CSV file
 *
 * @return {string}           - The output string
 */
const ParseCells = ( CSV, GRID, separator ) => {
	let output = [];
	let errors = [];

	CSV
		.split('\n')
		.forEach( ( line, i ) => {
			const cells = line.split( separator );
			const outputRow = [];
			const thisRow = i + 1;

			cells.forEach( ( cell, j ) => {
				const thisCol = GetCol( j );

				const thisCell = ParsePostfix( cell, GRID, `${ thisCol }${ thisRow }` );

				errors = [ ...errors, ...thisCell.errors ];
				outputRow.push( thisCell.expression );
			});

			output.push( outputRow.join( separator ) );

		});

	// removing duplicates
	errors = new Set( errors );
	errors = [ ...errors ];

	return {
		output: output.join('\n'),
		errors,
	};
};


/**
 * Open a CSV file and parse all postfix notations inside
 *
 * @param  {string} file      - The CSV content
 * @param  {string} separator - The CSV separator, optional, default: ","
 *
 * @return {string}           - The parsed output of the table
 */
const Postfixcsv = ( CSV, separator = ',' ) => {

	const GRID = MakeGrid( CSV, separator );
	const result = ParseCells( CSV, GRID, separator );

	return result;
};


/**
 * Export
 */
module.exports = exports = {
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
};
