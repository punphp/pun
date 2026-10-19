<?php

namespace App;

use Vendor\Pkg\Lib;
use Vendor\Pkg\Plain;
use Vendor\Pkg\SecretEnum;
use Vendor\Pkg\UsesTrait;

class Sub extends Lib
{
    public function calls(\ArrayObject $list, SecretEnum $enum, string $class): void
    {
        self::hiddenStatic();
        static::hiddenStatic();
        parent::hiddenStatic();
        Plain::make();
        $list->count();
        $enum->label();
        UsesTrait::fromTrait();
        $this->run();
        $unknown->hidden();
        $class::hiddenStatic();
        Plain::hiddenStatic();
        \Vendor\Pkg\SecretTrait::fromTrait();
    }
}
