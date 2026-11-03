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

use FFI\CData;
use FFI\CType;
use Symfony\Component\VarDumper\Cloner\Stub;

/**
 * Casts FFI extension classes to array representation.
 *
 * @author Nesmeyanov Kirill <nesk@xakep.ru>
 */
final class FFICaster
{
    /**
     * In case of "char*" contains a string, the length of which depends on
     * some other parameter, then during the generation of the string it is
     * possible to go beyond the allowable memory area.
     *
     * This restriction serves to ensure that processing does not take
     * up the entire allowable PHP memory limit.
     */
    private const MAX_STRING_LENGTH = 255;

    public static function castCTypeOrCData(CData|CType $data, array $args, Stub $stub): array
    {}

    private static function castFFIFunction(Stub $stub, CType $type): array
    {}

    private static function castFFIPointer(Stub $stub, CType $type, ?CData $data = null): array
    {}

    private static function castFFIStringValue(CData $data): string|CutStub
    {}

    private static function castFFIStructLike(CType $type, ?CData $data = null): array
    {}
}
