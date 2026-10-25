<?php
namespace GenericEmptyArrays;

/** @template K of array-key
 * @template V
 * @param array<K, V> $items
 * @return array<K, V>
 */
function keyed(array $items = []): array { return $items; }

/** @template T
 * @param list<T> $items
 * @return list<T>
 */
function listed(array $items = []): array { return $items; }

/** @template T
 * @param array{optional?: T} $items
 * @return T
 */
function shaped(array $items) { return $items['optional']; }

\PHPStan\dumpType(keyed());
\PHPStan\dumpType(keyed([]));
\PHPStan\dumpType(keyed(['key' => 37]));
\PHPStan\dumpType(listed());
\PHPStan\dumpType(listed([]));
\PHPStan\dumpType(listed([37]));
\PHPStan\dumpType(shaped([]));
\PHPStan\dumpType(new \ArrayObject());
\PHPStan\dumpType(new \ArrayObject([]));
\PHPStan\dumpType(new \ArrayObject(['key' => 37]));
\PHPStan\dumpType(new \ArrayIterator());
\PHPStan\dumpType(new \ArrayIterator([]));

class ReadonlyStorage
{
    /** @var \ArrayObject<string, int> */
    private readonly \ArrayObject $storage;

    public function __construct()
    {
        $local = new \ArrayObject();
        $this->storage = $local;
        $this->storage['a'] = 1;
        $this->storage['a'] = 2;
        \PHPStan\dumpType($this->storage);
        unset($this->storage['a']);
        unset($local['a']);
    }
}
