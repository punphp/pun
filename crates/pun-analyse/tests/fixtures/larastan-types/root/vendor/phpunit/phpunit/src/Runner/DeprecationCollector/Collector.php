<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner\DeprecationCollector;

use PHPUnit\Event\Facade;
use PHPUnit\Event\Test\DeprecationTriggered;
use PHPUnit\TestRunner\IssueFilter;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class Collector
{
    private readonly IssueFilter $issueFilter;

    /**
     * @var list<non-empty-string>
     */
    private array $deprecations = [];

    /**
     * @var list<non-empty-string>
     */
    private array $filteredDeprecations = [];

    public function __construct(Facade $facade, IssueFilter $issueFilter)
    {}

    /**
     * @return list<non-empty-string>
     */
    public function deprecations(): array
    {}

    /**
     * @return list<non-empty-string>
     */
    public function filteredDeprecations(): array
    {}

    public function testPrepared(): void
    {}

    public function testTriggeredDeprecation(DeprecationTriggered $event): void
    {}

    /**
     * The deprecation is not added to the filtered deprecations, which count
     * towards stopping the test run. A deprecation of collected events is
     * added to them when, and only if, the collected events are forwarded,
     * which notifies this collector through testTriggeredDeprecation().
     */
    public function testTriggeredDeprecationWhileEventsAreCollected(DeprecationTriggered $event): void
    {}
}
