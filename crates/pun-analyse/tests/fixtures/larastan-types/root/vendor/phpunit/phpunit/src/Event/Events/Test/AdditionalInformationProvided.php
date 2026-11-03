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
use PHPUnit\Event\Code\TestMethod;
use PHPUnit\Event\Event;
use PHPUnit\Event\Telemetry;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final readonly class AdditionalInformationProvided implements Event
{
    private Telemetry\Info $telemetryInfo;
    private TestMethod $test;

    /**
     * @var non-empty-string
     */
    private string $additionalInformation;

    /**
     * @param non-empty-string $additionalInformation
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public function __construct(Telemetry\Info $telemetryInfo, TestMethod $test, string $additionalInformation)
    {}

    public function telemetryInfo(): Telemetry\Info
    {}

    public function test(): TestMethod
    {}

    /**
     * @return non-empty-string
     */
    public function additionalInformation(): string
    {}

    /**
     * @return non-empty-string
     */
    public function asString(): string
    {}
}
