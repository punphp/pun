<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-timer.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\Timer;

use function is_float;
use function memory_get_peak_usage;
use function microtime;
use function sprintf;

final class ResourceUsageFormatter
{
    /**
     * @var array<string,int>
     */
    private const array SIZES = [
        'GB' => 1073741824,
        'MB' => 1048576,
        'KB' => 1024,
    ];

    public function resourceUsage(Duration $duration): string
    {}

    /**
     * @throws TimeSinceStartOfRequestNotAvailableException
     */
    public function resourceUsageSinceStartOfRequest(): string
    {}

    private function bytesToString(int $bytes): string
    {}
}
