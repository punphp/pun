<?php
namespace UnsetRequiredCrossFile;
use UnsetRequiredParents\ReadonlyFirst;
use UnsetRequiredParents\HookedFirst;
use UnsetRequiredParents\Diamond;
function properties(ReadonlyFirst $readonly, HookedFirst $hooked, Diamond $diamond): void
{
    unset($readonly->p);
    unset($hooked->p);
    unset($diamond->p);
}
