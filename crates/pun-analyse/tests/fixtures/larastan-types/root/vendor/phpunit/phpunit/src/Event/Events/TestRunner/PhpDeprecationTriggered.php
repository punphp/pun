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

use const PHP_EOL;
use function implode;
use function sprintf;
use PHPUnit\Event\Code\IssueTrigger\IssueTrigger;
use PHPUnit\Event\Event;
use PHPUnit\Event\Telemetry;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final readonly class PhpDeprecationTriggered implements Event
{
    private Telemetry\Info $telemetryInfo;

    /**
     * @var non-empty-string
     */
    private string $message;

    /**
     * @var non-empty-string
     */
    private string $file;

    /**
     * @var positive-int
     */
    private int $line;
    private bool $suppressed;
    private bool $ignoredByBaseline;
    private bool $ignoredByFilter;
    private IssueTrigger $trigger;

    /**
     * @param non-empty-string $message
     * @param non-empty-string $file
     * @param positive-int     $line
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public function __construct(Telemetry\Info $telemetryInfo, string $message, string $file, int $line, bool $suppressed, bool $ignoredByBaseline, bool $ignoredByFilter, IssueTrigger $trigger)
    {}

    public function telemetryInfo(): Telemetry\Info
    {}

    /**
     * @return non-empty-string
     */
    public function message(): string
    {}

    /**
     * @return non-empty-string
     */
    public function file(): string
    {}

    /**
     * @return positive-int
     */
    public function line(): int
    {}

    public function wasSuppressed(): bool
    {}

    public function ignoredByBaseline(): bool
    {}

    public function ignoredByFilter(): bool
    {}

    public function trigger(): IssueTrigger
    {}

    /**
     * @return non-empty-string
     */
    public function asString(): string
    {}
}
