<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Event\TestData;

use function count;
use Countable;
use IteratorAggregate;

/**
 * @template-implements IteratorAggregate<non-negative-int, TestData>
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final readonly class TestDataCollection implements Countable, IteratorAggregate
{
    /**
     * @var list<TestData>
     */
    private array $data;
    private ?DataFromDataProvider $fromDataProvider;

    /**
     * @param list<TestData> $data
     */
    public static function fromArray(array $data): self
    {}

    private function __construct(TestData ...$data)
    {}

    /**
     * @return list<TestData>
     */
    public function asArray(): array
    {}

    public function count(): int
    {}

    /**
     * @phpstan-assert-if-true !null $this->fromDataProvider
     */
    public function hasDataFromDataProvider(): bool
    {}

    /**
     * @throws NoDataSetFromDataProviderException
     */
    public function dataFromDataProvider(): DataFromDataProvider
    {}

    public function getIterator(): TestDataCollectionIterator
    {}
}
