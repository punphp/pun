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
 * Casts SPL related classes to array representation.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 *
 * @final
 *
 * @internal
 */
class SplCaster
{
    private const SPL_FILE_OBJECT_FLAGS = [
        \SplFileObject::DROP_NEW_LINE => 'DROP_NEW_LINE',
        \SplFileObject::READ_AHEAD => 'READ_AHEAD',
        \SplFileObject::SKIP_EMPTY => 'SKIP_EMPTY',
        \SplFileObject::READ_CSV => 'READ_CSV',
    ];

    public static function castArrayObject(\ArrayObject $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castArrayIterator(\ArrayIterator $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castHeap(\Iterator $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castDoublyLinkedList(\SplDoublyLinkedList $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castFileInfo(\SplFileInfo $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castFileObject(\SplFileObject $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castObjectStorage(\SplObjectStorage $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castOuterIterator(\OuterIterator $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castWeakReference(\WeakReference $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castWeakMap(\WeakMap $c, array $a, Stub $stub, bool $isNested): array
    {}

    private static function castSplArray(\ArrayObject|\ArrayIterator $c, array $a, Stub $stub, bool $isNested): array
    {}
}
