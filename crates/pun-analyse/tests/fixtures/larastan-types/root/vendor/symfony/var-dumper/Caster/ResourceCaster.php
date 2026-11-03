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
 * Casts common resource types to array representation.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 *
 * @final
 *
 * @internal
 */
class ResourceCaster
{
    public static function castDba(\Dba\Connection $dba, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castProcess($process, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castStream($stream, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castStreamContext($stream, array $a, Stub $stub, bool $isNested): array
    {}
}
