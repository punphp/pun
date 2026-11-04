<?php

namespace iamcal;

class SQLParserSyntaxException extends \Exception { }

class SQLParser{

	#
	# the main public interface is very simple
	#

	public $tokens = array();
	public $tables = array();
	public $source_map = array();

	public $find_single_table = false;
	public $throw_on_bad_syntax = false;

	public $sql;

	public function parse($sql){}

	#
	# lex and collapse tokens
	#
	public function lex($sql) {}

	#
	# simple lexer based on http://www.contrib.andrew.cmu.edu/~shadow/sql/sql1992.txt
	#
	# returns an array of [position, len] tuples for each token

	private function _lex($sql){}


	function walk($tokens, $sql, $source_map){}

	private function generateTableKey(array $table){}

	function parse_create_table($tokens, $i, $num){}


	function next_tokens($tokens, $i){ func_get_args(); }

	function parse_create_definition($tokens, &$i){}

	function parse_field_or_key(&$tokens, &$fields, &$indexes){}

	function slice_until_next_field($tokens, &$i){}

	function parse_field($tokens){}

	function parse_table_props($tokens, &$i){}


	# Given the source map, extract the tokens from the original sql,
	# Along the way, simplify parsing by merging certain tokens when
	# they occur next to each other. MySQL treats these productions
	# equally: 'UNIQUE|UNIQUE INDEX|UNIQUE KEY' and if they are
	# all always a single token it makes parsing easier.

	function _extract_tokens($sql, &$source_map){}

	function parse_index_mode(&$tokens, &$index){}

	function parse_index_columns(&$tokens, &$index){}

	function parse_index_options(&$tokens, &$index){}


	#
	# helper functions for parsing bits of field definitions
	#

	function parse_field_length(&$tokens, &$f){}

	function parse_field_length_decimals(&$tokens, &$f){}

	function parse_field_binary(&$tokens, &$f){}

	function parse_field_unsigned(&$tokens, &$f){}

	function parse_field_zerofill(&$tokens, &$f){}

	function parse_field_charset(&$tokens, &$f){}

	function parse_field_collate(&$tokens, &$f){}

	function parse_value_list(&$tokens){}

	function decode_identifier($token){}

	function decode_value($token){}
}


