<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Schema;

use function count;


/**
 * Translates the type-expression language of Expect::type() ('int|string', '?Foo', 'int[]', 'string:1..5',
 * 'pattern:\d+') into a plain description.
 * @internal
 */
final class TypeExpression
{
	/**
	 * The names Validators::is() knows and what kind of value they are; 'scalar' is a union handled apart,
	 * any other name is a class.
	 */
	private const Names = [
		'mixed' => Kind::Any,
		'bool' => Kind::Bool, 'boolean' => Kind::Bool,
		'int' => Kind::Int, 'integer' => Kind::Int,
		'float' => Kind::Float, 'number' => Kind::Number,
		'string' => Kind::String,
		'unicode' => Kind::String, 'email' => Kind::String, 'uri' => Kind::String, 'url' => Kind::String,
		'identifier' => Kind::String, 'class' => Kind::String, 'interface' => Kind::String,
		'file' => Kind::String, 'directory' => Kind::String, 'alnum' => Kind::String, 'alpha' => Kind::String,
		'digit' => Kind::String, 'lower' => Kind::String, 'upper' => Kind::String, 'space' => Kind::String,
		'xdigit' => Kind::String, 'pattern' => Kind::String,
		'array' => Kind::Array, 'list' => Kind::List, 'iterable' => Kind::Iterable,
		'object' => Kind::Object, 'callable' => Kind::Callable,
		'resource' => Kind::Other, 'none' => Kind::Other, 'numeric' => Kind::Other, 'numericint' => Kind::Other,
	];

	/** @var array<string, array<string, mixed>> */
	private static array $cache = [];


	/** @return array<string, mixed> */
	public static function parse(string $expression): array
	{}


	/** @return array<string, mixed> */
	private static function doParse(string $expression): array
	{}


	/**
	 * One alternative of the expression; 'scalar' expands to several.
	 * @return list<array<string, mixed>>
	 */
	private static function parseAlternative(string $part): array
	{}


	/**
	 * Bounds written as 'min..max', '..max', 'min..' or an exact 'value'.
	 * @return array{int|float|null, int|float|null}
	 */
	private static function parseRange(string $range): array
	{}


	/**
	 * An integer bound stays an integer, a float would lose the precision of large ones.
	 */
	private static function parseBound(string $bound): int|float
	{}
}
