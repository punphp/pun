<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Utils;

use Nette;
use function count, is_resource, is_string;


/**
 * PHP type reflection.
 */
final readonly class Type
{
	/** @var list<string|self> */
	private array $types;
	private ?string $singleName;
	private string $kind; // | &


	/**
	 * Creates a Type object based on reflection. Resolves self, static and parent to the actual class name.
	 * If the subject has no type, it returns null.
	 */
	public static function fromReflection(
		\ReflectionFunctionAbstract|\ReflectionParameter|\ReflectionProperty $reflection,
	): ?self
	{}


	/** @return ($asObject is true ? self : self|string) */
	private static function fromReflectionType(
		\ReflectionType $type,
		\ReflectionFunctionAbstract|\ReflectionParameter|\ReflectionProperty $of,
		bool $asObject,
	): self|string
	{}


	/**
	 * Creates the Type object according to the text notation.
	 */
	public static function fromString(string $type): self
	{}


	/**
	 * Creates a Type object based on the actual type of value.
	 */
	public static function fromValue(mixed $value): self
	{}


	/**
	 * Resolves 'self', 'static' and 'parent' to the actual class name.
	 */
	public static function resolve(
		string $type,
		\ReflectionFunction|\ReflectionMethod|\ReflectionParameter|\ReflectionProperty $of,
	): string
	{}


	/** @param  array<string|self>  $types */
	private function __construct(array $types, string $kind = '|')
	{}


	public function __toString(): string
	{}


	/**
	 * Returns a union type that accepts both the current type and the given type.
	 */
	public function with(string|self $type): self
	{}


	/**
	 * Returns the array of subtypes that make up the compound type as strings.
	 * @return list<string|array<string|array<mixed>>>
	 */
	public function getNames(): array
	{}


	/**
	 * Returns the array of subtypes that make up the compound type as Type objects.
	 * @return list<self>
	 */
	public function getTypes(): array
	{}


	/**
	 * Returns the type name for simple types, otherwise null.
	 */
	public function getSingleName(): ?string
	{}


	/**
	 * Returns true whether it is a union type.
	 */
	public function isUnion(): bool
	{}


	/**
	 * Returns true whether it is an intersection type.
	 */
	public function isIntersection(): bool
	{}


	/**
	 * Checks whether it is a simple (non-compound) type. Single nullable types such as ?int are also considered simple.
	 */
	public function isSimple(): bool
	{}


	#[\Deprecated('use isSimple()')]
	public function isSingle(): bool
	{}


	/**
	 * Checks whether it is a simple PHP built-in type (int, string, bool, etc.).
	 */
	public function isBuiltin(): bool
	{}


	/**
	 * Checks whether it is a simple class or interface name (not a built-in type).
	 */
	public function isClass(): bool
	{}


	/**
	 * Determines if type is special class name self/parent/static.
	 */
	public function isClassKeyword(): bool
	{}


	/**
	 * Checks whether a value of the given type could be assigned to this type.
	 */
	public function allows(string|self $type): bool
	{}


	/** @param array<string>  $givenTypes */
	private function allowsAny(array $givenTypes): bool
	{}


	/**
	 * @param array<string>  $ourTypes
	 * @param array<string>  $givenTypes
	 */
	private function allowsAll(array $ourTypes, array $givenTypes): bool
	{}
}
