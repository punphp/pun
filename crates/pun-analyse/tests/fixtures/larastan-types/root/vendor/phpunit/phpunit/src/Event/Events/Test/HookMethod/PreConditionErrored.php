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
use PHPUnit\Event\Code;
use PHPUnit\Event\Code\Throwable;
use PHPUnit\Event\Event;
use PHPUnit\Event\Telemetry;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final readonly class PreConditionErrored implements Event
{
    private Telemetry\Info $telemetryInfo;
    private Code\TestMethod $test;
    private Code\ClassMethod $calledMethod;
    private Throwable $throwable;

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public function __construct(Telemetry\Info $telemetryInfo, Code\TestMethod $test, Code\ClassMethod $calledMethod, Throwable $throwable)
    {}

    public function telemetryInfo(): Telemetry\Info
    {}

    public function test(): Code\TestMethod
    {}

    public function calledMethod(): Code\ClassMethod
    {}

    public function throwable(): Throwable
    {}

    /**
     * @return non-empty-string
     */
    public function asString(): string
    {}
}
