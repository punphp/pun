<?php

namespace Rector\Tests\TypeDeclarationDocblocks\Rector\ClassMethod\AddReturnDocblockForCommonObjectDenominatorRector\Pun;

final class SkipAnonymousClass
{
    public function middleware()
    {
        return [new class { public function handle($command, $next) { return $next($command); } }];
    }
}
