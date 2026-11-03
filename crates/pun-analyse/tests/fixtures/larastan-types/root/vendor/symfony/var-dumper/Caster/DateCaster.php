<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\VarDumper\Caster;

use Symfony\Component\VarDumper\Cloner\Stub;

/**
 * Casts DateTimeInterface related classes to array representation.
 *
 * @author Dany Maillard <danymaillard93b@gmail.com>
 *
 * @final
 *
 * @internal
 */
class DateCaster
{
    private const PERIOD_LIMIT = 3;

    public static function castDateTime(\DateTimeInterface $d, array $a, Stub $stub, bool $isNested, int $filter): array
    {}

    public static function castInterval(\DateInterval $interval, array $a, Stub $stub, bool $isNested, int $filter): array
    {}

    private static function formatInterval(\DateInterval $i): string
    {}

    public static function castTimeZone(\DateTimeZone $timeZone, array $a, Stub $stub, bool $isNested, int $filter): array
    {}

    public static function castPeriod(\DatePeriod $p, array $a, Stub $stub, bool $isNested, int $filter): array
    {}

    private static function formatDateTime(\DateTimeInterface $d, string $extra = ''): string
    {}

    private static function formatSeconds(string $s, string $us): string
    {}
}
