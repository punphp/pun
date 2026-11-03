<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\VarDumper\Cloner;

use Symfony\Component\VarDumper\Caster\Caster;
use Symfony\Component\VarDumper\Dumper\ContextProvider\SourceContextProvider;

/**
 * @author Nicolas Grekas <p@tchwork.com>
 */
class Data implements \ArrayAccess, \Countable, \IteratorAggregate, \Stringable
{
    private array $data;
    private int $position = 0;
    private int|string $key = 0;
    private int $maxDepth = 20;
    private int $maxItemsPerDepth = -1;
    private int $useRefHandles = -1;
    private array $context = [];

    /**
     * @param array $data An array as returned by ClonerInterface::cloneVar()
     */
    public function __construct(array $data)
    {}

    public function getType(): ?string
    {}

    /**
     * Returns a native representation of the original value.
     *
     * @param array|bool $recursive Whether values should be resolved recursively or not
     *
     * @return string|int|float|bool|array|Data[]|null
     */
    public function getValue(array|bool $recursive = false): string|int|float|bool|array|null
    {}

    public function count(): int
    {}

    public function getIterator(): \Traversable
    { yield; }

    public function __get(string $key): mixed
    {}

    public function __isset(string $key): bool
    {}

    public function offsetExists(mixed $key): bool
    {}

    public function offsetGet(mixed $key): mixed
    {}

    public function offsetSet(mixed $key, mixed $value): void
    {}

    public function offsetUnset(mixed $key): void
    {}

    public function __toString(): string
    {}

    /**
     * Returns a depth limited clone of $this.
     */
    public function withMaxDepth(int $maxDepth): static
    {}

    /**
     * Limits the number of elements per depth level.
     */
    public function withMaxItemsPerDepth(int $maxItemsPerDepth): static
    {}

    /**
     * Enables/disables objects' identifiers tracking.
     *
     * @param bool $useRefHandles False to hide global ref. handles
     */
    public function withRefHandles(bool $useRefHandles): static
    {}

    public function withContext(array $context): static
    {}

    public function getContext(): array
    {}

    /**
     * Seeks to a specific key in nested data structures.
     */
    public function seek(string|int $key): ?static
    {}

    /**
     * Dumps data with a DumperInterface dumper.
     */
    public function dump(DumperInterface $dumper): void
    {}

    /**
     * Depth-first dumping of items.
     *
     * @param mixed $item A Stub object or the original value being dumped
     */
    private function dumpItem(DumperInterface $dumper, Cursor $cursor, array &$refs, mixed $item): void
    {}

    /**
     * Dumps children of hash structures.
     *
     * @return int The final number of removed items
     */
    private function dumpChildren(DumperInterface $dumper, Cursor $parentCursor, array &$refs, array $children, int $hashCut, int $hashType, bool $dumpKeys): int
    {}

    private function getStub(mixed $item): mixed
    {}
}
