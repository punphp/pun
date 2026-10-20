<?php

namespace CoverageClassTags;

trait SomeTrait
{
}

/**
 * @template T
 */
class Box
{
}

/**
 * @method SomeTrait first(int $a = null, $b = Box::NOPE)
 * @method int first(int $a = null)
 * @method T second<T of Missing>(T $value)
 * @method SomeTrait third()
 * @property SomeTrait $a
 * @property-read int $b
 * @property-write string $b
 * @property-write Box<int, int> $c
 * @property-read Missing $d
 * @mixin SomeTrait
 */
class Tagged
{
}

/**
 * @mixin int
 * @method Unknown traitMethod()
 * @property Unknown $traitProperty
 * @mixin \stdClass&\Exception
 */
trait TaggedTrait
{
}

class UsesTaggedTrait
{
	use TaggedTrait;
}

/**
 * @template T
 */
function generic(): void
{
	/** @var T $value */
	$value = 1;
	/** @var SomeTrait */
	$other = 2;
	/** @var Box<int, int> $box */
	$box = 3;
	if (class_exists(Guarded::class)) {
		/** @var Guarded $guarded */
		$guarded = 4;
	}
	echo $value, $other, $box;
	/** @var Missing $nop */
}

class Methods
{
	/**
	 * @template U
	 */
	public function method(): void
	{
		/** @var U $value */
		$value = 1;
		/** @var MissingToo $value */
	}

	/** @var MissingInClass $nop */
}

/**
 * @mixin \stdClass|int
 * @mixin \Countable&\Traversable
 * @mixin
 */
class MixinShapes
{
}

/** @var MissingAtEnd $nop */
