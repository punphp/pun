<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Event\Test;

use const PHP_EOL;
use function sprintf;
use function trim;
use PHPUnit\Event\Code;
use PHPUnit\Event\Code\ComparisonFailure;
use PHPUnit\Event\Code\Throwable;
use PHPUnit\Event\Event;
use PHPUnit\Event\Telemetry;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final readonly class AttemptFailed implements Event
{
    private Telemetry\Info $telemetryInfo;
    private Code\Test $test;
    private Throwable $throwable;
    private ?ComparisonFailure $comparisonFailure;
    private Telemetry\Duration $duration;

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public function __construct(Telemetry\Info $telemetryInfo, Code\Test $test, Throwable $throwable, ?ComparisonFailure $comparisonFailure, Telemetry\Duration $duration)
    {}

    public function telemetryInfo(): Telemetry\Info
    {}

    /**
     * The wall-clock time spent on this failed attempt.
     */
    public function duration(): Telemetry\Duration
    {}

    public function test(): Code\Test
    {}

    public function throwable(): Throwable
    {}

    /**
     * @phpstan-assert-if-true !null $this->comparisonFailure
     */
    public function hasComparisonFailure(): bool
    {}

    /**
     * @throws NoComparisonFailureException
     */
    public function comparisonFailure(): ComparisonFailure
    {}

    /**
     * @return non-empty-string
     */
    public function asString(): string
    {}
}
