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

use Ds\Collection;
use Ds\Map;
use Ds\Pair;
use Symfony\Component\VarDumper\Cloner\Stub;

/**
 * Casts Ds extension classes to array representation.
 *
 * @author Jáchym Toušek <enumag@gmail.com>
 *
 * @final
 */
class DsCaster
{
    public static function castCollection(Collection $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castMap(Map $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castPair(Pair $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castPairStub(DsPairStub $c, array $a, Stub $stub, bool $isNested): array
    {}
}
