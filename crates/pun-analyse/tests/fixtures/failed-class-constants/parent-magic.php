<?php

namespace FailedParentMagic;

function outside(): void
{
    $direct = parent::class;
    \PHPStan\dumpType($direct);
    $ordinary = parent::MISSING;
    \PHPStan\dumpType($ordinary);
    echo 'after-outside-parent';
}

class Parentless
{
    public function value(): void
    {
        $direct = parent::class;
        \PHPStan\dumpType($direct);
        $upper = PARENT::CLASS;
        \PHPStan\dumpType($upper);
        $_syntacticSelector = parent::{($syntacticName = 'class')};
        \PHPStan\dumpType($_syntacticSelector);
        \PHPStan\dumpType($syntacticName);
        $ordinary = parent::MISSING;
        \PHPStan\dumpType($ordinary);
        $holder = new parent();
        $dynamic = ($copy = $holder)::class;
        \PHPStan\dumpType($dynamic);
        $missing = $holder::MISSING;
        \PHPStan\dumpType($missing);
        $_selector = $holder::{($name = 'class')};
        \PHPStan\dumpType($_selector);
        \PHPStan\dumpType($name);
        echo 'after-parentless';
    }
}

class Base
{
    public const NUMBER = 37;
}

class Child extends Base
{
    public function value(): void
    {
        $direct = parent::class;
        \PHPStan\dumpType($direct);
        $ordinary = parent::NUMBER;
        \PHPStan\dumpType($ordinary);
        $missing = parent::MISSING;
        \PHPStan\dumpType($missing);
        $holder = new parent();
        $dynamic = ($copy = $holder)::NUMBER;
        \PHPStan\dumpType($dynamic);
        echo 'after-valid-parent';
    }
}

class Grandchild extends Child {}

function inherited(Grandchild $child): void
{
    $child->value();
    $ordinary = Grandchild::NUMBER;
    \PHPStan\dumpType($ordinary);
    echo 'after-inherited';
}
