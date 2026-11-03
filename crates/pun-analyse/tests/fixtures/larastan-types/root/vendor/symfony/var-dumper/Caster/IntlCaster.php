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
 * @author Nicolas Grekas <p@tchwork.com>
 * @author Jan Schädlich <jan.schaedlich@sensiolabs.de>
 *
 * @final
 *
 * @internal
 */
class IntlCaster
{
    public static function castMessageFormatter(\MessageFormatter $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castNumberFormatter(\NumberFormatter $c, array $a, Stub $stub, bool $isNested, int $filter = 0): array
    {}

    public static function castIntlTimeZone(\IntlTimeZone $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castIntlCalendar(\IntlCalendar $c, array $a, Stub $stub, bool $isNested, int $filter = 0): array
    {}

    public static function castIntlDateFormatter(\IntlDateFormatter $c, array $a, Stub $stub, bool $isNested, int $filter = 0): array
    {}

    private static function castError(object $c, array $a): array
    {}
}
