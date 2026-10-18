<?php

namespace SignatureTemplates;

use Countable as Counted;

interface Entity {}

/** @template E of Entity */
interface Repository {}

class Model {}

class User extends Model {}

/** @template T */
class Box {}

class Invokable
{
	public function __invoke(): void {}
}

/**
 * @template E of Entity
 * @template T of Repository<E>
 * @template U
 * @param class-string<T> $class
 */
function bounds(string $class): void {}

/**
 * @template T = Model
 * @template-covariant V
 * @phpstan-template W of Model = User
 * @param array<int, V> $a
 */
function defaults(array $a): void {}

/**
 * @template T
 * @param int $x
 * @phpstan-param T $x
 */
function prefixed($x): void {}

/**
 * @template T
 * @param T $x
 */
function nativeInt(int $x): void {}

/**
 * @template T of User
 * @param T $x
 */
function subclass(Model $x): void {}

/**
 * @template T of Model
 * @param T $x
 */
function superclass(User $x): void {}

/**
 * @template T
 * @param T $x
 */
function nativeMixed(mixed $x): void {}

/**
 * @template T of object
 * @param T $x
 */
function objectBound(Model $x): void {}

/**
 * @template T of \Countable
 * @param T $x
 */
function builtinBound(\ArrayObject $x): void {}

/**
 * @template T of Counted
 * @param T $x
 */
function importedBound(Counted $x): void {}

/**
 * @template T
 * @param T|null $x
 */
function nullableNative(?int $x): void {}

/**
 * @template T
 * @param int|null $y
 * @param T|null $x
 */
function nullDefault(int $x = null, int $y = 1): void {}

/**
 * @template T
 * @param Box<T> $box
 */
function generic(Box $box): void {}

/**
 * @template T
 * @param callable(T): void $cb
 */
function callableParam(callable $cb): void {}

/**
 * @template T
 * @param Invokable|callable(): T $cb
 */
function invokable(callable $cb): void {}

/**
 * @template T
 * @param Model|callable(): T $cb
 */
function notInvokable(callable $cb): void {}

/**
 * @template T
 * @param array{a: T, ...<int, string>} $a
 */
function shape(array $a): void {}

/**
 * @template T
 * @param array{a: int, ...<T, string>} $a
 */
function unsealedKey(array $a): void {}

/**
 * @template T
 * @param array{a: int, ...<int, T>} $a
 */
function unsealedValue(array $a): void {}

/**
 * @template T
 * @param object{a: T} $o
 */
function objectShape(object $o): void {}

/**
 * @template T
 * @param (T is int ? string : int) $x
 */
function conditionalParam($x): void {}

/**
 * @template T
 * @param list<T> ...$xs
 */
function variadic(array ...$xs): void {}

/**
 * @template T
 * @param T &$x
 */
function byRef(&$x): void {}

/**
 * @template T
 * @param array<
 *   int,
 *   T
 * > $x
 */
function multiLine(array $x): void {}

/**
 * @template T
 * @return ($x is int ? T : string)
 */
function conditionalNative(int $x): string {}

/**
 * @template T
 * @return ($x is int ? T : string)
 */
function conditionalWithoutNative(int $x) {}

/**
 * @template T
 * @param not a type $x
 */
function unparsable($x): void {}

/**
 * @template T
 */
function unreferenced(): void {}

/**
 * @template T
 * @param ?T $x
 */
function nullablePhpDoc(Model $x): void {}

/**
 * @template T of Model
 * @param ?T $x
 */
function nullablePhpDocNullableNative(?Model $x): void {}

/**
 * @template T
 * @param T&\Countable $x
 */
function intersectionPhpDoc(Model $x): void {}

/**
 * @template T of User
 * @param T|int|null $x
 */
function unionNative(Model|int $x): void {}

/**
 * @template T of int
 * @param T $x
 */
function disjointNative(string $x): void {}

/**
 * @template T of Model
 * @param T $x
 */
function dnfNative((Model&\Countable)|null $x): void {}

/**
 * @template T of Model
 * @param T $x
 */
function intersectionNative(Model&\Countable $x): void {}

/**
 * @template T
 * @param \Closure|callable(): T $cb
 */
function closure(callable $cb): void {}

/**
 * @template T
 * @param Model&callable(): T $cb
 */
function callableIntersection(callable $cb): void {}

/**
 * @template T
 * @param T[] $x
 */
function arrayOf(array $x): void {}

/**
 * @template T
 * @param Unknown|callable(): T $cb
 */
function unknownCallable(callable $cb): void {}

/**
 * @template T
 * @param ?callable(): T $cb
 */
function nullableCallable(callable $cb): void {}

/**
 * @template T
 * @param callable-string|array{0: T} $cb
 */
function shapeCallable(callable $cb): void {}

class Holder
{
	/**
	 * @template T of self
	 * @param T $x
	 */
	public function selfNative(self $x): void {}

	/**
	 * @template T
	 * @psalm-template S
	 * @param T $t
	 */
	public function method($t, $s): void {}
}
