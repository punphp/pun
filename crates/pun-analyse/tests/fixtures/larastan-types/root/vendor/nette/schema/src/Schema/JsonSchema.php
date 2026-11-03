<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Schema;

use Nette;
use function count, is_bool, is_float, is_int, is_string;


/**
 * Exports a schema as JSON Schema.
 */
final class JsonSchema
{
	private const Formats = ['email' => 'email', 'url' => 'uri', 'uri' => 'uri'];


	/**
	 * @return array<string, mixed>|\stdClass
	 * @throws Nette\NotSupportedException  when the schema contains a class type or another PHP-only type
	 */
	public static function export(Schema $schema): array|\stdClass
	{}


	/**
	 * @param  Schema|array<string, mixed>  $schema  a schema, or a description as describe() returns it
	 * @return array<string, mixed>|\stdClass
	 */
	private static function build(Schema|array $schema): array|\stdClass
	{}


	/**
	 * @param  Schema|array<string, mixed>  $schema
	 * @return array<string, mixed>
	 */
	private static function describe(Schema|array $schema): array
	{}


	/**
	 * @param  Schema|array<string, mixed>|null  $schema
	 * @return array<string, mixed>|\stdClass
	 */
	private static function buildOrAny(Schema|array|null $schema): array|\stdClass
	{}


	/**
	 * @param  array<string, mixed>  $item
	 * @return array<string, mixed>
	 */
	private static function buildString(array $item): array
	{}


	/**
	 * A list that wraps a single value accepts the item alone as well.
	 * @param  array<string, mixed>  $item
	 * @return array<string, mixed>
	 */
	private static function buildList(array $item): array
	{}


	/**
	 * A PHP array is a JSON array when its keys are integers and an object otherwise.
	 * @param  array<string, mixed>  $item
	 * @return array<string, mixed>
	 */
	private static function buildArray(array $item): array
	{}


	/**
	 * @param  array<string, mixed>  $item
	 * @return array<string, mixed>
	 */
	private static function buildStructure(array $item): array
	{}


	/**
	 * A required position makes all positions before it present too.
	 * @param  array<string, mixed>  $item
	 * @return array<string, mixed>
	 */
	private static function buildTuple(array $item): array
	{}


	/**
	 * Whether a missing item is an error. A structure is created from its defaults instead,
	 * so it fails only when something inside it is required or its size has a minimum.
	 * @param  Schema|array<string, mixed>  $schema
	 */
	private static function isRequired(Schema|array $schema): bool
	{}


	/**
	 * The type is stated only when every value shares it.
	 * @param  list<mixed>  $values
	 * @return array<string, mixed>
	 */
	private static function buildEnum(array $values): array
	{}


	/**
	 * Scalar values of an anyOf() come first as one enum, then the schema variants.
	 * @param  array<string, mixed>  $item
	 * @return array<string, mixed>
	 */
	private static function buildUnion(array $item): array
	{}


	/**
	 * A count of characters, items or properties is a non-negative integer.
	 * @param  array<string, mixed>  $item
	 * @return array<string, int>
	 */
	private static function countRange(array $item, string $minKey, string $maxKey): array
	{}


	/**
	 * @param  array<string, mixed>  $item
	 * @return array<string, int|float>
	 */
	private static function range(array $item, string $minKey, string $maxKey): array
	{}
}
