<?php
namespace UnsetRequiredDeclarations;
class Native
{
    public readonly int $p;
    public function __construct() { $this->p = 1; }
}
class Hooked { public int $p { get => 1; } }
/** @phpstan-require-extends Native */
interface ReadonlyContract {}
/** @phpstan-require-extends Hooked */
interface HookedContract {}

namespace UnsetRequiredParents;
use UnsetRequiredDeclarations\ReadonlyContract as ZReadonly;
use UnsetRequiredDeclarations\HookedContract as AHooked;
interface ReadonlyFirst extends ZReadonly, AHooked {}
interface HookedFirst extends AHooked, ZReadonly {}
interface Diamond extends ReadonlyFirst, HookedFirst {}
