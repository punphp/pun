<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Logging;

use const FILE_APPEND;
use const LOCK_EX;
use const PHP_EOL;
use const PHP_OS_FAMILY;
use function explode;
use function file_put_contents;
use function implode;
use function str_repeat;
use function str_replace;
use function strlen;
use PHPUnit\Event\Event;
use PHPUnit\Event\Tracer\Tracer;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class EventLogger implements Tracer
{
    private string $path;
    private bool $includeTelemetryInfo;

    public function __construct(string $path, bool $includeTelemetryInfo)
    {}

    public function trace(Event $event): void
    {}

    private function telemetryInfo(Event $event): string
    {}
}
