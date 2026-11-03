<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Event\TestRunner;

use function sprintf;
use PHPUnit\Event\Event;
use PHPUnit\Event\Telemetry;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final readonly class StaticAnalysisForCodeCoverageFinished implements Event
{
    private Telemetry\Info $telemetryInfo;

    /**
     * @var non-negative-int
     */
    private int $cacheHits;

    /**
     * @var non-negative-int
     */
    private int $cacheMisses;

    /**
     * @param non-negative-int $cacheHits
     * @param non-negative-int $cacheMisses
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public function __construct(Telemetry\Info $telemetryInfo, int $cacheHits, int $cacheMisses)
    {}

    public function telemetryInfo(): Telemetry\Info
    {}

    /**
     * @return non-negative-int
     */
    public function cacheHits(): int
    {}

    /**
     * @return non-negative-int
     */
    public function cacheMisses(): int
    {}

    /**
     * @return non-empty-string
     */
    public function asString(): string
    {}
}
