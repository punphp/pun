<?php

namespace ThrowsBoundaries;

/** @throws bool */
class C
{
    /** @throws string */
    public const VALUE = 1;

    /** @throws float */
    public int $plain = 1;

    /** @throws bool */
    public int $hooked {
        /** @throws string */
        get => 1;
        /** @throws float */
        set { $this->plain = $value; }
    }

    /** @throws int */
    public function method(): void {}

    /** @throws string */
}

/** @throws int */
interface I {}

/** @throws string */
trait T
{
    /** @throws bool */
    public function fromTrait(): void {}
}

class UsesTrait { use T; }

/** @throws bool */
enum E { case A; }

/** @throws string */
function statements(bool $flag): void
{
    /** @throws int */
    $a = 1;
    /** @throws bool */
    strlen('x');
    /** @throws float */
    echo 'x';
    /** @throws int */
    if ($flag) {
        /** @throws string */
        $a = 2;
    }
    /** @throws bool */
    else {
        /** @throws string */
        $a = 3;
    }
    /** @throws bool */
    try { $a = 4; }
    /** @throws string */
    catch (\Exception $e) {}
    /** @throws float */
    finally { $a = 5; }
    /** @throws int */
    return;
    /** @throws string */
    $a = 6;
}

/** @throws bool */
$closure = function (): void {};
$closure2 = /** @throws string */ function (): void {};
$arrow = /** @throws float */ fn() => 1;

/** @throws int */
const GLOBAL_VALUE = 1;

/** @throws bool */
;

/** @throws bool */
new class {};

$anonymous = new /** @throws string */ class {};

$readonly = new
    /** @throws float */
    readonly class {};

$attributed = new
    /** @throws int */
    #[\AllowDynamicProperties]
    class {};

/** @throws string */
