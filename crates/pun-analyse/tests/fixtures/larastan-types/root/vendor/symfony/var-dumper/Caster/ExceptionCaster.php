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

use Symfony\Component\ErrorHandler\Exception\FlattenException;
use Symfony\Component\ErrorHandler\Exception\SilencedErrorContext;
use Symfony\Component\VarDumper\Cloner\Stub;
use Symfony\Component\VarDumper\Exception\ThrowingCasterException;

/**
 * Casts common Exception classes to array representation.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 *
 * @final
 *
 * @internal
 */
class ExceptionCaster
{
    public static int $srcContext = 1;
    public static bool $traceArgs = true;
    public static array $errorTypes = [
        \E_DEPRECATED => 'E_DEPRECATED',
        \E_USER_DEPRECATED => 'E_USER_DEPRECATED',
        \E_RECOVERABLE_ERROR => 'E_RECOVERABLE_ERROR',
        \E_ERROR => 'E_ERROR',
        \E_WARNING => 'E_WARNING',
        \E_PARSE => 'E_PARSE',
        \E_NOTICE => 'E_NOTICE',
        \E_CORE_ERROR => 'E_CORE_ERROR',
        \E_CORE_WARNING => 'E_CORE_WARNING',
        \E_COMPILE_ERROR => 'E_COMPILE_ERROR',
        \E_COMPILE_WARNING => 'E_COMPILE_WARNING',
        \E_USER_ERROR => 'E_USER_ERROR',
        \E_USER_WARNING => 'E_USER_WARNING',
        \E_USER_NOTICE => 'E_USER_NOTICE',
        2048 => 'E_STRICT',
    ];

    private static array $framesCache = [];

    public static function castError(\Error $e, array $a, Stub $stub, bool $isNested, int $filter = 0): array
    {}

    public static function castException(\Exception $e, array $a, Stub $stub, bool $isNested, int $filter = 0): array
    {}

    public static function castErrorException(\ErrorException $e, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castThrowingCasterException(ThrowingCasterException $e, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castSilencedErrorContext(SilencedErrorContext $e, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castTraceStub(TraceStub $trace, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castFrameStub(FrameStub $frame, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castFlattenException(FlattenException $e, array $a, Stub $stub, bool $isNested): array
    {}

    private static function filterExceptionArray(string $xClass, array $a, string $xPrefix, int $filter): array
    {}

    private static function traceUnshift(array &$trace, ?string $class, string $file, int $line): void
    {}

    private static function extractSource(string $srcLines, int $line, int $srcContext, string $lang, ?string $file, array $frame): EnumStub
    {}
}
