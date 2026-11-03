<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\Configuration;

use function count;
use Countable;
use IteratorAggregate;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @immutable
 *
 * @template-implements IteratorAggregate<non-negative-int, FilterFile>
 */
final readonly class FilterFileCollection implements Countable, IteratorAggregate
{
    /**
     * @var list<FilterFile>
     */
    private array $files;

    /**
     * @param list<FilterFile> $files
     */
    public static function fromArray(array $files): self
    {}

    private function __construct(FilterFile ...$files)
    {}

    /**
     * @return list<FilterFile>
     */
    public function asArray(): array
    {}

    public function count(): int
    {}

    public function notEmpty(): bool
    {}

    public function getIterator(): FilterFileCollectionIterator
    {}
}
