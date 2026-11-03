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
use Ramsey\Uuid\Math\CalculatorInterface;
use Ramsey\Uuid\Math\RoundingMode;
use Ramsey\Uuid\Type\Hexadecimal;
use Ramsey\Uuid\Type\Integer as IntegerObject;
use Ramsey\Uuid\Type\Time;

use function explode;
use function str_pad;

use const STR_PAD_LEFT;

/**
 * UnixTimeConverter converts Unix Epoch timestamps to/from hexadecimal values consisting of milliseconds elapsed since
 * the Unix Epoch
 *
 * @immutable
 */
class UnixTimeConverter implements TimeConverterInterface
{
    private const MILLISECONDS = 1000;

    public function __construct(private CalculatorInterface $calculator)
    {}

    public function calculateTime(string $seconds, string $microseconds): Hexadecimal
    {}

    public function convertTime(Hexadecimal $uuidTimestamp): Time
    {}
}
