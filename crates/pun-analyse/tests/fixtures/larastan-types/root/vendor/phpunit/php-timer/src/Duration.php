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

use function floor;
use function sprintf;

/**
 * @immutable
 */
final readonly class Duration
{
    private float $nanoseconds;
    private int $hours;
    private int $minutes;
    private int $seconds;
    private int $milliseconds;

    public static function fromMicroseconds(float $microseconds): self
    {}

    public static function fromNanoseconds(float $nanoseconds): self
    {}

    private function __construct(float $nanoseconds)
    {}

    public function asNanoseconds(): float
    {}

    public function asMicroseconds(): float
    {}

    public function asMilliseconds(): float
    {}

    public function asSeconds(): float
    {}

    public function asString(): string
    {}
}
