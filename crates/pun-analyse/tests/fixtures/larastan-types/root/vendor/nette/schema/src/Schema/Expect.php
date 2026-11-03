<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Schema;

use Nette;
use Nette\Schema\Elements\{AnyOf, ArrayType, EnumType, NumberType, StringType, Structure, Type};
use function array_key_exists, count, is_object;


/**
 * Schema generator.
 *
 * @method static Type scalar($default = null)
 * @method static StringType string($default = null)
 * @method static NumberType int($default = null)
 * @method static NumberType float($default = null)
 * @method static Type bool($default = null)
 * @method static Type null()
 * @method static ArrayType list($default = [])
 * @method static Type mixed($default = null)
 * @method static StringType email($default = null)
 * @method static StringType unicode($default = null)
 */
final class Expect
{
	/** @param  array<int|string, mixed>  $args */
	public static function __callStatic(string $name, array $args): Type
	{}


	/**
	 * Creates a schema for a type expression (e.g., 'int|string', 'null|float', 'email'); an expression
	 * of a single kind of value gets its dedicated StringType, NumberType or ArrayType. An enum class means
	 * instanceof, as any class; Expect::enum() accepts its backing values.
	 */
	public static function type(string $type): Type
	{}


	/**
	 * The subclass of Type that Expect::type() returns for a parsed expression: the dedicated one when
	 * every variant is of its kind, the plain Type otherwise.
	 * @param  array<string, mixed>  $item
	 * @return class-string<Type>
	 * @internal
	 */
	public static function typeClass(array $item): string
	{}


	/**
	 * Creates a schema for a backed enum: a case or its backing value is accepted, the case is returned.
	 * @param  class-string<\BackedEnum>  $enum
	 */
	public static function enum(string $enum): EnumType
	{}


	/**
	 * Creates a union schema that accepts any of the given values or sub-schemas.
	 */
	public static function anyOf(mixed ...$set): AnyOf
	{}


	/**
	 * Creates a structure schema with defined properties; output is stdClass.
	 * @param  Schema[]  $shape
	 */
	public static function structure(array $shape): Structure
	{}


	/**
	 * Generates a structure schema from a class instance by reflecting its properties or constructor parameters.
	 * @param  array<string, Schema>  $items  Optional overrides for specific properties.
	 */
	public static function from(object $object, array $items = []): Structure
	{}


	/**
	 * Creates an array schema. When passed Schema elements, behaves like structure() but outputs an array.
	 * Without Schema elements, creates a plain array type with the given default value.
	 * @param  mixed[]  $shape
	 */
	public static function array(?array $shape = []): Structure|ArrayType
	{}


	/**
	 * Creates an associative or indexed array schema where every value matches the given type.
	 */
	public static function arrayOf(string|Schema $valueType, string|Schema|null $keyType = null): ArrayType
	{}


	/**
	 * Creates a list schema (sequentially indexed from 0) where every element matches the given type.
	 * With $wrap, a single value is also accepted and wrapped into a one-item list.
	 */
	public static function listOf(string|Schema $type, bool $wrap = false): ArrayType
	{}


	/**
	 * Creates a fixed-size array where each position has its own schema; a later layer replaces the tuple wholesale.
	 * @param  Schema[]  $shape
	 */
	public static function tuple(array $shape): Elements\TupleType
	{}
}
