'use strict';


/**
 * Parse CLI arguments
 *
 * @param  {array} args - The arguments coming from the node process, default: process.argv
 *
 * @return {object}     - All arguments parsed into a settings object
 */
module.exports = exports = ( args = process.argv ) => {
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


