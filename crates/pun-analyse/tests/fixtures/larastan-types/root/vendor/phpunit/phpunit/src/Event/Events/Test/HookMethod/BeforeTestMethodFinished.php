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
use PHPUnit\Event\Event;
use PHPUnit\Event\Telemetry;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final readonly class BeforeTestMethodFinished implements Event
{
    private Telemetry\Info $telemetryInfo;
    private Code\TestMethod $test;

    /**
     * @var list<Code\ClassMethod>
     */
    private array $calledMethods;

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public function __construct(Telemetry\Info $telemetryInfo, Code\TestMethod $test, Code\ClassMethod ...$calledMethods)
    {}

    public function telemetryInfo(): Telemetry\Info
    {}

    public function test(): Code\TestMethod
    {}

    /**
     * @return list<Code\ClassMethod>
     */
    public function calledMethods(): array
    {}

    /**
     * @return non-empty-string
     */
    public function asString(): string
    {}
}
