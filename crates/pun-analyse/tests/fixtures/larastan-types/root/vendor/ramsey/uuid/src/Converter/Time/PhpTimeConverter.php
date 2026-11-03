<?php

/**
 * This file is part of the ramsey/uuid library
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 *
 * @copyright Copyright (c) Ben Ramsey <ben@benramsey.com>
 * @license http://opensource.org/licenses/MIT MIT
 */

declare(strict_types=1);

namespace Ramsey\Uuid\Converter\Time;

use Ramsey\Uuid\Converter\TimeConverterInterface;
use Ramsey\Uuid\Math\BrickMathCalculator;
use Ramsey\Uuid\Math\CalculatorInterface;
use Ramsey\Uuid\Type\Hexadecimal;
use Ramsey\Uuid\Type\Integer as IntegerObject;
use Ramsey\Uuid\Type\Time;

use function count;
use function dechex;
use function explode;
use function is_float;
use function is_int;
use function str_pad;
use function strlen;
use function substr;

use const STR_PAD_LEFT;
use const STR_PAD_RIGHT;

/**
 * PhpTimeConverter uses built-in PHP functions and standard math operations available to the PHP programming language
 * to provide facilities for converting parts of time into representations that may be used in UUIDs
 *
 * @immutable
 */
class PhpTimeConverter implements TimeConverterInterface
{
    /**
     * The number of 100-nanosecond intervals from the Gregorian calendar epoch to the Unix epoch.
     */
    private const GREGORIAN_TO_UNIX_INTERVALS = 0x01b21dd213814000;

    /**
     * The number of 100-nanosecond intervals in one second.
     */
    private const SECOND_INTERVALS = 10_000_000;

    /**
     * The number of 100-nanosecond intervals in one microsecond.
     */
    private const MICROSECOND_INTERVALS = 10;

    private int $phpPrecision;
    private CalculatorInterface $calculator;
    private TimeConverterInterface $fallbackConverter;

    public function __construct(
        ?CalculatorInterface $calculator = null,
        ?TimeConverterInterface $fallbackConverter = null,
    ) {}

    public function calculateTime(string $seconds, string $microseconds): Hexadecimal
    {}

    public function convertTime(Hexadecimal $uuidTimestamp): Time
    {}

    /**
     * @param float | int $time The time to split into seconds and microseconds
     *
     * @return string[]
     *
     * @pure
     */
    private function splitTime(float | int $time): array
    {}
}
