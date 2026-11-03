<?php declare(strict_types=1);
/*
 * This file is part of sebastian/comparator.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\Comparator;

use const PHP_VERSION;
use function array_unshift;
use function extension_loaded;
use function version_compare;
use SebastianBergmann\Exporter\Exporter;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for sebastian/comparator
 */
final class Factory
{
    private static ?Factory $instance = null;

    /**
     * @var array<non-negative-int, Comparator>
     */
    private array $customComparators = [];

    /**
     * @var list<Comparator>
     */
    private array $defaultComparators = [];

    /** @var positive-int */
    private int $contextLines               = 3;
    private bool $closureComparisonOccurred = false;
    private Exporter $exporter;

    public static function getInstance(): self
    {}

    public function __construct()
    {}

    /**
     * @return positive-int
     */
    public function contextLines(): int
    {}

    /**
     * @param positive-int $contextLines
     */
    public function setContextLines(int $contextLines): void
    {}

    public function exporter(): Exporter
    {}

    public function setExporter(Exporter $exporter): void
    {}

    /**
     * @internal this method is called by ClosureComparator and is not part of the consumer-facing API
     */
    public function recordClosureComparison(): void
    {}

    public function closureComparisonOccurred(): bool
    {}

    public function resetClosureComparisonTracking(): void
    {}

    public function getComparatorFor(mixed $expected, mixed $actual): Comparator
    {}

    /**
     * Registers a new comparator.
     *
     * This comparator will be returned by getComparatorFor() if its accept() method
     * returns TRUE for the compared values. It has higher priority than the
     * existing comparators, meaning that its accept() method will be invoked
     * before those of the other comparators.
     */
    public function register(Comparator $comparator): void
    {}

    /**
     * Unregisters a comparator.
     *
     * This comparator will no longer be considered by getComparatorFor().
     */
    public function unregister(Comparator $comparator): void
    {}

    public function reset(): void
    {}

    private function registerDefaultComparators(): void
    {}

    private function registerDefaultComparator(Comparator $comparator): void
    {}
}
