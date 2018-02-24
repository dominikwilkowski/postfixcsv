'use strict';


/**
 * Dependencies
 */
const Path = require('path');
const Fs = require('fs');


/**
 * Parse CLI arguments
 *
 * @param  {array} args - The arguments coming from the node process, default: process.argv
 *
 * @return {object}     - All arguments parsed into a settings object
 */
const ParseArgs = ( args = process.argv ) => {
	const _hasSeparator = args.includes('-s') && args[ args.indexOf('-s') + 1 ];

	return {
		file: args[ 2 ]
			? args[ 2 ]
			: void( 0 ),
		separator: _hasSeparator
			? args[ args.indexOf('-s') + 1 ]
			: void( 0 ),
	};
};


/**
 * Promisified reading a file
 *
 * @param  {string} location - The location of the file to be read
 *
 * @return {promise object}  - The content of the file
 */
const ReadFile = ( location ) => {
	return new Promise( ( resolve, reject ) => {
		Fs.readFile( Path.normalize( location ), `utf8`, ( error, content ) => {
			if( error ) {
				console.error(`Reading file failed for >>${ location }<<`);
				console.error( JSON.stringify( error ) );

				reject( error );
			}
			else {
				resolve( content );
			}
		});
	});
};


/**
 * Export
 */
module.exports = exports = {
	ReadFile,
	ParseArgs,
};

