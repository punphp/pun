<?php declare(strict_types=1);

namespace MaxInheritedConstructor;

class Model {}

/**
 * @template TKey of array-key
 * @template-covariant TValue
 */
interface Enumerable
{
    /** @param iterable<TKey, TValue>|null $items */
    public function __construct($items = []);
}

/**
 * @template TKey of array-key
 * @template-covariant TValue
 * @implements Enumerable<TKey, TValue>
 */
class BaseCollection implements Enumerable
{
    /** @param iterable<TKey, TValue>|null $items */
    public function __construct($items = []) {}
}

/**
 * @template TKey of array-key
 * @template TModel of Model
 * @extends BaseCollection<TKey, TModel>
 */
class Collection extends BaseCollection {}

// the inherited constructor's templates resolve through the parent class, not the interface beside it
/** @return Collection<int, Model> */
function emptyCollection(): Collection
{
    \PHPStan\dumpType(new Collection());

    return new Collection();
}
