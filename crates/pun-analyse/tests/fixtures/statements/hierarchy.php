<?php

namespace Hierarchy;

/** @final */
final class FinalBoth
{
}

class ExtendsFinalBoth extends FinalBoth
{
}

#[\AllowDynamicProperties]
/** @final */
class FinalByDocWithAttribute
{
}

class ExtendsFinalByDocWithAttribute extends FinalByDocWithAttribute
{
}

class ExtendsClosure extends \Closure
{
}

class Base
{
}

trait UsesNonTraits
{
	use Base;
	use MissingTrait;
}

class UsesUsesNonTraits
{
	use UsesNonTraits;
}

class UsesClass
{
	use Base;
}

/**
 * @final
 * @phpstan-consistent-constructor
 */
class FinalByDocConsistent
{
	private function __construct()
	{
	}
}

/** @psalm-consistent-constructor */
class PsalmConsistent
{
	private function __construct()
	{
	}
}

/** @consistent-constructor */
class PlainConsistent
{
	protected function __construct()
	{
	}
}

/**
 * @psalm-inheritors Base
 */
trait InheritorsTrait
{
}

/**
 * @phpstan-sealed
 */
trait EmptySealedTrait
{
}

class CountsNothing implements \Countable
{
}

trait AbstractFoo
{
	abstract public function foo(): void;
}

class ParentFoo
{
	public function foo(): void
	{
	}
}

class ChildFoo extends ParentFoo
{
	use AbstractFoo;
}

class OnlyTraitFoo
{
	use AbstractFoo;
}

abstract class AbstractParent
{
	abstract public function bar(): void;
}

class ConcreteChild extends AbstractParent
{
}

class CountableIterator extends \ArrayIterator implements \Countable
{
}

class SerializableObject extends \ArrayObject implements \Serializable
{
}

class SerializableWithMagic implements \Serializable
{
	public function serialize()
	{
	}

	public function unserialize($data)
	{
	}

	public function __serialize(): array
	{
		return [];
	}

	public function __unserialize(array $data): void
	{
	}
}

trait CallableProperty
{
	public ?callable $callback;
}

class UsesCallableProperty
{
	use CallableProperty;

	public \Closure|callable $either;

	public (\Countable&\Traversable)|callable $dnf;
}

class BreakIterator extends \IntlBreakIterator
{
}

class XmlElement extends \SimpleXMLElement
{
}

interface WithBodylessMethod
{
	public function noBody(): void;
}

function instanceofTrait(mixed $value): bool
{
	return $value instanceof AbstractFoo;
}

/**
 * @method void magic()
 */
interface WithMagicMethod
{
}

class ImplementsMagicMethod implements WithMagicMethod
{
}

class Filter extends \FilterIterator
{
}

trait AbstractCount
{
	abstract public function count(): int;
}

class CountedArrayObject extends \ArrayObject
{
	use AbstractCount;
}

interface Dated
{
	public static function tomorrow(): static;
}

/**
 * @method static static tomorrow()
 */
trait ComparesDates
{
}

trait CreatesDates
{
	public static function tomorrow(): static
	{
		throw new \LogicException();
	}
}

trait DateBehaviour
{
	use ComparesDates;
	use CreatesDates;
}

class Date implements Dated
{
	use DateBehaviour;
}

class SubDate extends Date
{
}
