<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner\TestIndex;

use function preg_match;
use PHPUnit\Runner\Filter\CompiledNameFilter;

/**
 * Decides whether a test file can be skipped for a run that selects tests by
 * their name.
 *
 * Unlike a selection by group, a selection by name can only be decided for a
 * test that has no data set: NameFilterIterator matches the filter against the
 * entire name of a test, including the name of its data set, and the name of a
 * data set is not known until the data provider has been invoked (#6741). A
 * file that has a single test method with data sets is therefore always loaded.
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class NameFilterPruner
{
    private ?CompiledNameFilter $filter;

    public static function fromFilter(string $filter): self
    {}

    public static function withoutFilter(): self
    {}

    private function __construct(?CompiledNameFilter $filter)
    {}

    /**
     * @phpstan-assert-if-true !null $this->filter
     */
    public function prunes(): bool
    {}

    public function canSkip(TestIndexEntry $entry): bool
    {}
}
