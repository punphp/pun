<?php

namespace App;

use Vendor\Pkg\Child;
use Vendor\Pkg\Implementer;
use Vendor\Pkg\Lib;
use Vendor\Pkg\Secret;
use Vendor\Pkg\SecretContract;

class Consumer
{
    public function run(Lib $lib, Secret $secret, SecretContract $contract, ?Lib $maybe, Implementer $impl): void
    {
        $lib->hidden();
        $lib->open();
        $lib
            ->hidden();
        Lib::hiddenStatic();
        $secret->run();
        Secret::make();
        Child::make();
        $contract->contract();
        $impl->contract();
        $maybe?->hidden();
        $lib::hiddenStatic();
    }
}
