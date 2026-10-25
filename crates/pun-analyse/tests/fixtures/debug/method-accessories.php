<?php

namespace MethodAccessories;

class OpenType
{
    public function nativeMethod(): void {}

    public static function probe(bool $flag = false): void
    {
        \PHPStan\dumpType(static::class);
        if (method_exists(static::class, 'MiXeD')) {
            \PHPStan\dumpType(static::class);
            \PHPStan\dumpPhpDocType(static::class);
            \PHPStan\dumpType(self::class);
            if (method_exists(static::class, 'Second')) {
                \PHPStan\dumpType(static::class);
            }
        } else {
            \PHPStan\dumpType(static::class);
        }
        \PHPStan\dumpType(static::class);
        if (method_exists(self::class, 'SelfOnly')) {
            \PHPStan\dumpType(self::class);
            \PHPStan\dumpType(static::class);
        }
        if (method_exists(static::class, 'left') || $flag) {
            \PHPStan\dumpType(static::class);
        }
        if (method_exists(static::class, 'First') && method_exists(static::class, 'Last')) {
            \PHPStan\dumpType(static::class);
        }
        if (!method_exists(static::class, 'AfterExit')) {
            return;
        }
        \PHPStan\dumpType(static::class);
    }
}

/** @param class-string<OpenType> $class */
function strings(string $class, string $plain): void
{
    if (method_exists($class, 'CuStOm')) {
        \PHPStan\dumpType($class);
        \PHPStan\dumpPhpDocType($class);
        \PHPStan\dumpType($plain);
    }
    if (method_exists($plain, 'NoNarrowing')) {
        \PHPStan\dumpType($plain);
    }
    if (method_exists((OpenType::class), 'Literal')) {
        \PHPStan\dumpType(OpenType::class);
        \PHPStan\dumpPhpDocType(OpenType::class);
    }
}

function objects(OpenType $object, OpenType $other, object $unknown, mixed $mixed): void
{
    if (method_exists($object, 'ExTrA')) {
        \PHPStan\dumpType($object);
        \PHPStan\dumpPhpDocType($object);
        \PHPStan\dumpType($other);
    } else {
        \PHPStan\dumpType($object);
    }
    \PHPStan\dumpType($object);
    if (method_exists($unknown, '__INVOKE')) {
        \PHPStan\dumpType($unknown);
    }
    if (method_exists($mixed, 'AnyMethod')) {
        \PHPStan\dumpType($mixed);
        \PHPStan\dumpPhpDocType($mixed);
    }
    if (method_exists($object, 'NATIVEmethod')) {
        \PHPStan\dumpType($object);
    }
}

/** @method void virtualMethod() */
class Annotated {}

function annotation(Annotated $object): void
{
    if (method_exists($object, 'virtualMethod')) {
        \PHPStan\dumpType($object);
    }
}

/** @param class-string<Annotated> $class */
function annotationString(string $class): void
{
    if (method_exists($class, 'virtualMethod')) {
        \PHPStan\dumpType($class);
    }
}

function dynamic(OpenType $object, string $method): void
{
    if (method_exists($object, $method)) {
        \PHPStan\dumpType($object);
    }
    $checked = method_exists($object, 'Stored');
    if ($checked) {
        \PHPStan\dumpType($object);
    }
}

function spelling(OpenType $Object, OpenType $object, bool $flag): void
{
    if (method_exists($Object, 'MiXeD')) {
        \PHPStan\dumpType($Object);
        \PHPStan\dumpType($object);
    }
    if (method_exists($object, 'mixed')) {
        \PHPStan\dumpType($object);
    }
    if (method_exists($object, 'One') && method_exists($object, 'one')) {
        \PHPStan\dumpType($object);
    }
    if (method_exists($object, 'Same') || method_exists($object, 'Same')) {
        \PHPStan\dumpType($object);
    }
    if ($flag) {
        \PHPStan\dumpType($object);
    } elseif (method_exists($object, 'ElseIf')) {
        \PHPStan\dumpType($object);
    } else {
        \PHPStan\dumpType($object);
    }
}
