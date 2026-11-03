<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TestRunner\TestResult;

use function array_filter;
use function count;
use function str_contains;
use PHPUnit\Event\Facade as EventFacade;
use PHPUnit\Runner\DeprecationCollector\Facade as DeprecationCollectorFacade;
use PHPUnit\TestRunner\IssueFilter;
use PHPUnit\TextUI\Configuration\Configuration;
use PHPUnit\TextUI\Configuration\Registry as ConfigurationRegistry;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class Facade
{
    private static ?Collector $collector = null;
    private static bool $interrupted     = false;

    public static function init(): void
    {}

    public static function interrupt(): void
    {}

    public static function wasInterrupted(): bool
    {}

    public static function result(): TestResult
    {}

    public static function shouldStop(): bool
    {}

    private static function collector(): Collector
    {}

    private static function stopOnDeprecation(Configuration $configuration): bool
    {}
}
