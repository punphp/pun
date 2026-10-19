<?php

namespace Hier;

trait T
{
    const A = 1;
    const A = 1;
    public $p;
    public $p;
    public function m() {}
    public function M() {}
}

trait Unused
{
    public function m() {}
    public function m() {}
}

class C1
{
    use T;
}

class C2
{
    use T;
}

interface I
{
    private function a();
    protected function b();
    public function c();
    #[\Deprecated]
    protected static function d();
}

/** @phpstan-sealed S1 */
class Sealed {}

class S1 extends Sealed {}

class S2 extends Sealed {}

class S5 extends S1 {}

/** @phpstan-sealed S1|S3 */
interface SI {}

class S3 implements SI {}

class S4 implements SI, \Countable
{
    public function count(): int
    {
        return 0;
    }
}

enum E: string implements SI
{
    case A = 'a';
}

interface SubI extends SI {}

/**
 * @psalm-inheritors Late
 * @phpstan-sealed Early
 */
abstract class Both {}

class Early extends Both {}

class Late extends Both {}

/** @phpstan-sealed Sealed */
abstract class SealedSelf {}

class Plain extends SealedSelf {}

$anonymous = new class extends Sealed {};
$pair = [new class extends Sealed {}, new class extends Sealed {}];
