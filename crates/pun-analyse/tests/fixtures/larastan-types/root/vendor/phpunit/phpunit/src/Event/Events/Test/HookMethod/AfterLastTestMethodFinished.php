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
use PHPUnit\Framework\TestCase;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final readonly class AfterLastTestMethodFinished implements Event
{
    private Telemetry\Info $telemetryInfo;

    /**
     * @var class-string<TestCase>
     */
    private string $testClassName;

    /**
     * @var list<Code\ClassMethod>
     */
    private array $calledMethods;

    /**
     * @param class-string<TestCase> $testClassName
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public function __construct(Telemetry\Info $telemetryInfo, string $testClassName, Code\ClassMethod ...$calledMethods)
    {}

    public function telemetryInfo(): Telemetry\Info
    {}

    /**
     * @return class-string<TestCase>
     */
    public function testClassName(): string
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
