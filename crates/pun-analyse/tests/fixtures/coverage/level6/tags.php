<?php

namespace CoverageTags;

/**
 * @template T
 */
class Generic
{
}

/**
 * @method array plain(array $a, $b = null, int ...$rest)
 * @method static array make(array &$ref, string $s = 'x,y', array $n = NULL)
 * @method static fluent()
 * @method Generic<int> typed()
 * @method T templated<T of array>(T $t, Generic $g)
 * @method array<K, V> pair<K, V of array>(K $k, V $v)
 * @method static static fluentStatic()
 * @method static array<T> staticGeneric<T = int>()
 * @method bad(
 * @method
 * @property array $both
 * @property-read Generic $read
 * @property-write array $write
 * @property Generic<int> $fine
 * @mixin Generic
 * @mixin array
 * @mixin \ArrayAccess
 */
class Tagged
{
}

/**
 * @method array own()
 * @property array $prop
 * @mixin \Traversable
 */
interface TaggedInterface
{
}

/**
 * @method array own()
 */
trait TaggedTrait
{
}

/**
 * @method array own()
 */
enum TaggedEnum
{
}

/**
 * @extends \ArrayIterator<int, string>
 */
class Iter extends \ArrayIterator
{
}

class RawIter extends \ArrayIterator implements \IteratorAggregate
{
    public function getIterator(): \Iterator
    {
        return $this;
    }
}

/**
 * @implements \IteratorAggregate<int, string>
 */
class Agg implements \IteratorAggregate
{
    public function getIterator(): \Iterator
    {
        return new \ArrayIterator([]);
    }
}

class Plain extends \Exception
{
}
