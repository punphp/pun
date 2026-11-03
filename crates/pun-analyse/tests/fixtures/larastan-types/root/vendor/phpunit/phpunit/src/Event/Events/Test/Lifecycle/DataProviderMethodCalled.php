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
use PHPUnit\Event\Code\ClassMethod;
use PHPUnit\Event\Event;
use PHPUnit\Event\Telemetry\Info;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final readonly class DataProviderMethodCalled implements Event
{
    private Info $telemetryInfo;
    private ClassMethod $testMethod;
    private ClassMethod $dataProviderMethod;

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public function __construct(Info $telemetryInfo, ClassMethod $testMethod, ClassMethod $dataProviderMethod)
    {}

    public function telemetryInfo(): Info
    {}

    public function testMethod(): ClassMethod
    {}

    public function dataProviderMethod(): ClassMethod
    {}

    /**
     * @return non-empty-string
     */
    public function asString(): string
    {}
}
