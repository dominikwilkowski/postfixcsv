#!/usr/bin/env node
/***************************************************************************************************************************************************************
 *
 * Parse a spreadsheet with postfix notations from your CLI
 *
 * @example
 *   postfixcsv [file]       # [file] is the csv file to be parsed
 *   postfixcsv [file] -s \; # the optional flag "-s" takes one argument to adjust what the csv separator is
 *                           # ( remember to escape a semicolon in most shells )
 *
 **************************************************************************************************************************************************************/

'use strict';


/**
 * Dependencies
 */
const Postfixcsv = require('./src/postfixcsv.js').Postfixcsv;
const CleanCSV = require('./src/postfixcsv.js').CleanCSV;
const ParseArgs = require('./src/cli.js').ParseArgs;
const ReadFile = require('./src/cli.js').ReadFile;
const Path = require('path');
const Fs = require('fs');


const SETTINGS = ParseArgs();

const fileLocation = Path.normalize(`${ process.cwd() }/${ SETTINGS.file }`);

if( !Fs.existsSync( fileLocation ) ) {
	console.error(`Cannot find file at >>${ fileLocation }<<`);

	return;
}

( async function () {
	try {
		const CSV = CleanCSV( await ReadFile( fileLocation ) );

		const postfix = Postfixcsv( CSV, SETTINGS.separator );

		console.log();
		console.log('The parsed output of the supplied CSV file is:');

		if( postfix.errors.length > 0 ) {
			console.log();
			console.error( postfix.errors.join('\n') );
			console.log();
		}

		console.log( postfix.output );
		console.log();
	}
	catch( error ) {
		console.log( error );
	}
})();
