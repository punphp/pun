<?php

namespace DeadCode\NullCoalesce;

/**
 * @property string $magic
 */
class Holder implements \ArrayAccess
{
	public int $typed;

	public int $typedDefault = 1;

	/** @var string */
	public $phpDoc = '';

	/** @var null */
	public $alwaysNull;

	public ?int $nullable = null;

	public static int $staticTyped = 1;

	/** @var array{a: int, b?: string} */
	public array $shape = ['a' => 1];

	public Holder $self;

	public null $nullNative;

	public function __construct(
		public readonly int $promoted,
		public int $promotedMutable,
	) {
		$this->self = $this;
	}

	public function offsetExists(mixed $offset): bool
	{
		return true;
	}

	public function offsetGet(mixed $offset): mixed
	{
		return 1;
	}

	public function offsetSet(mixed $offset, mixed $value): void
	{
	}

	public function offsetUnset(mixed $offset): void
	{
	}

	public function properties(): void
	{
		echo $this->typed ?? 0;
		echo $this->typedDefault ?? 0;
		echo $this->phpDoc ?? 0;
		echo $this->alwaysNull ?? 0;
		echo $this->nullable ?? 0;
		echo $this->magic ?? 0;
		echo $this->promoted ?? 0;
		echo $this->promotedMutable ?? 0;
		echo self::$staticTyped ?? 0;
		echo static::$staticTyped ?? 0;
		echo $this->self->typedDefault ?? 0;
		echo $this->shape['a'] ?? 0;
		echo $this->shape['b'] ?? 0;
		echo $this->undeclared ?? 0;
		$this->typed = 1;
		echo $this->typed ?? 0;
		$this->nullNative = null;
		echo $this->nullNative ?? 0;
	}
}

class Child extends Holder
{
	public function parentStatic(): void
	{
		echo parent::$staticTyped ?? 0;
	}
}

function variables(int $i, ?int $n, mixed $m): void
{
	echo $i ?? 0;
	echo $n ?? 0;
	echo $m ?? 0;
	echo $undefined ?? 0;
	if ($i > 0) {
		$maybe = 1;
	}
	echo $maybe ?? 0;
	$get = $_GET ?? 0;
	echo $_SESSION ?? 0;
	$null = null;
	echo $null ?? 0;
	$i ??= 1;
	$undefinedAssign ??= 1;
	echo ($i) ?? 0;
	echo $$m ?? 0;
	foreach ([1, 2] as $v) {
		echo $v ?? 0;
		$later = 1;
	}
	echo $later ?? 0;
}

/**
 * @param array<string, int> $map
 * @param list<int> $list
 * @param non-empty-list<int> $nonEmptyList
 * @param array{1: int, 2: int} $intKeys
 * @param array{a: int}|array{b: int} $union
 * @param array{0: int}|non-empty-list<int> $unionShape
 * @param 'abc'|array{0: int} $stringOrShape
 * @param array{a: int, ...<string, int>} $open
 */
function offsets(string|array $stringOrShape, array $open, array $unionShape, array $map, array $list, array $nonEmptyList, array $intKeys, array $union, string $s, int $int, Holder $holder, \stdClass $object, mixed $mixed, string $key): void
{
	$shape = ['a' => 1, 'b' => null, 'c' => ['d' => 1]];
	echo $shape['a'] ?? 0;
	echo $shape['b'] ?? 0;
	echo $shape['x'] ?? 0;
	echo $shape['x']['y'] ?? 0;
	echo $shape['c']['d'] ?? 0;
	echo $shape['c']['e'] ?? 0;
	echo $shape[$key] ?? 0;
	echo $map['a'] ?? 0;
	echo $map[1] ?? 0;
	echo $list[0] ?? 0;
	echo $list['a'] ?? 0;
	echo $nonEmptyList[0] ?? 0;
	echo $intKeys[$key] ?? 0;
	echo $intKeys['1'] ?? 0;
	echo $intKeys[true] ?? 0;
	echo $intKeys[1.5] ?? 0;
	echo $intKeys[null] ?? 0;
	echo $union['a'] ?? 0;
	echo $unionShape[0] ?? 0;
	echo $stringOrShape[0] ?? 0;
	echo $open['b'] ?? 0;
	echo $s[0] ?? 0;
	echo $s['a'] ?? 0;
	echo $int[0] ?? 0;
	echo $holder['a'] ?? 0;
	echo $object['a'] ?? 0;
	echo $mixed['a'] ?? 0;
	echo $undefinedArray['a'] ?? 0;
	echo $undefinedArray['a']['b'] ?? 0;
	$shape['a'] ??= 1;
	$map['a'] ??= 1;
	$null = null;
	echo $null['a'] ?? 0;
}

function fetches(Holder $holder, ?Holder $maybeHolder, Holder|\stdClass $union, string $name, ?string $str): void
{
	echo $str?->bar ?? 0;
	echo $holder->typedDefault ?? 0;
	echo $holder->phpDoc ?? 0;
	echo $holder->nullable ?? 0;
	echo $holder->magic ?? 0;
	echo $holder->$name ?? 0;
	echo $maybeHolder->typedDefault ?? 0;
	echo $maybeHolder?->typedDefault ?? 0;
	echo $holder?->typedDefault ?? 0;
	echo $holder?->{$name} ?? 0;
	echo $union->typedDefault ?? 0;
	echo Holder::$staticTyped ?? 0;
	echo $holder::$staticTyped ?? 0;
	echo $undefinedHolder->typed ?? 0;
	echo $undefinedHolder::$staticTyped ?? 0;
	echo Missing::$x ?? 0;
	echo $holder->self->typedDefault ?? 0;
	echo $holder->self->typed ?? 0;
	echo $undefinedHolder->self->typed ?? 0;
	echo $holder->shape['a'] ?? 0;
	$holder->typedDefault ??= 1;
}

function expressions(): void
{
	echo rand() ?? 0;
	echo null ?? 0;
	echo strlen('a') ?? 0;
	echo (rand() > 1 ? 1 : null) ?? 0;
	$a = (new Holder(1, 1)) ?? 'foo';
	(new Holder(1, 1)) ?? 'foo';
}

function maybeInt(): ?int
{
	return null;
}

/**
 * @param array{a: int|null} $nullableShape
 */
function unnecessary(array $nullableShape, int $i, ?int $n, Holder $holder): void
{
	echo $i ?? null;
	echo $n ?? null;
	echo $undefined ?? null;
	echo $i ?? NULL;
	echo $i ?? rand();
	echo $holder->typedDefault ?? null;
	echo $holder->typed ?? null;
	echo $holder->magic ?? null;
	$shape = ['a' => 1];
	echo $shape['a'] ?? null;
	echo $shape[$i] ?? null;
	$s = 'a';
	echo $s[0] ?? null;
	echo Holder::$staticTyped ?? null;
	echo $holder->self->typedDefault ?? null;
	echo strlen('a') ?? null;
	echo $nullableShape['a'] ?? null;
	echo maybeInt() ?? null;
}

echo $globalScope ?? 0;

trait CoalesceTrait
{
	public function inTrait(): void
	{
		$s = 'a';
		echo $s ?? 0;
		echo $this->prop ?? 0;
		echo $this->same ?? 0;
	}
}

class UsesTraitA
{
	use CoalesceTrait;

	public int $prop = 1;

	public int $same = 1;
}

class UsesTraitB
{
	use CoalesceTrait;

	public ?int $prop = 1;

	public int $same = 1;
}
