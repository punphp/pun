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

use function sprintf;
use PHPUnit\Event\Code;
use PHPUnit\Event\Event;
use PHPUnit\Event\Telemetry;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final readonly class Finished implements Event
{
    private Telemetry\Info $telemetryInfo;
    private Code\Test $test;

    /**
     * @var non-negative-int
     */
    private int $numberOfAssertionsPerformed;

    /**
     * @param non-negative-int $numberOfAssertionsPerformed
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public function __construct(Telemetry\Info $telemetryInfo, Code\Test $test, int $numberOfAssertionsPerformed)
    {}

    public function telemetryInfo(): Telemetry\Info
    {}

    public function test(): Code\Test
    {}

    /**
     * @return non-negative-int
     */
    public function numberOfAssertionsPerformed(): int
    {}

    /**
     * @return non-empty-string
     */
    public function asString(): string
    {}
}
