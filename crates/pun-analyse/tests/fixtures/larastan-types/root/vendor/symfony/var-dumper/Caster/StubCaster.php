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
 * Casts a caster's Stub.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 *
 * @final
 *
 * @internal
 */
class StubCaster
{
    public static function castStub(Stub $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castCutArray(CutArrayStub $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function cutInternals($obj, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castClassDump(ClassDumpStub $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castEnum(EnumStub $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castScalar(ScalarStub $scalarStub, array $a, Stub $stub): array
    {}
}
