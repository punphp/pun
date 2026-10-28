<?php declare(strict_types=1);

namespace BranchDeclarationEffects;

function symmetric(bool $flag): void { if ($flag) { $x = 1; } else { $x = 2; } }
function partial(bool $flag): void { if ($flag) { $x = 1; } }
function comparison(string $name): void { if ($name === 'first') { $x = 1; } else { $x = 2; } }
function relational(int $number): void { if ($number >= 5) { $x = 1; } else { $x = 2; } }
function negated(bool $flag): void { if (!$flag) { $x = 1; } }
function nested(bool $a, bool $b): void { if ($a) { if ($b) { $x = 1; } } }
function elseifBranches(bool $a, bool $b): void { if ($a) { $x = 1; } elseif ($b) { $x = 2; } else { $x = 3; } }
function colonBranches(bool $a, bool $b): void { if ($a): $x = 1; elseif ($b): $x = 2; else: $x = 3; endif; }
function leftEffect(bool $flag): void { if ($flag) { echo 'effect'; } else { $x = 1; } }
function rightEffect(bool $flag): void { if ($flag) { $x = 1; } else { echo 'effect'; } }
function elseifEffect(bool $a, bool $b): void { if ($a) { $x = 1; } elseif ($b) { echo 'effect'; } }
function conditionEffect(): void { if (print 'effect') { $x = 1; } }
function elseifConditionEffect(bool $flag): void { if ($flag) { $x = 1; } elseif (print 'effect') { $x = 2; } }
function conditionWrite(bool $flag): void { if ($local = $flag) { $x = 1; } }
function conditionGlobal(bool $flag): void { if ($GLOBALS['branch'] = $flag) { $x = 1; } }
function bothReturn(bool $flag): void {
    if ($flag) { return; } else { return; }
    echo 'unreachable';
}
function onlyLeftReturns(bool $flag): void {
    if ($flag) { return; } else { $x = 1; }
    echo 'reachable';
}
function onlyRightReturns(bool $flag): void {
    if ($flag) { $x = 1; } else { return; }
    echo 'reachable';
}
function omittedElse(bool $flag): void {
    if ($flag) { return; }
    echo 'reachable';
}
function elseifReturns(bool $a, bool $b): void {
    if ($a) { return; } elseif ($b) { return; } else { return; }
    echo 'unreachable';
}
function alwaysTrue(): void { if (true) { $x = 1; } else { echo 'unreachable'; } }
function alwaysFalse(): void { if (false) { echo 'unreachable'; } else { $x = 1; } }
function falseWithoutElse(): void { if (false) { echo 'unreachable'; } }
function trueReturn(): void { if (true) { return; } echo 'unreachable'; }
function falseReturn(): void { if (false) { return; } echo 'reachable'; }
function unreachableUnknown(): void { if (false) { $object = new \stdClass(); } else { $x = 1; } }
function blockedElseif(): void { if (true) { $x = 1; } elseif (print 'condition') { echo 'unreachable'; } }
function trueElseif(bool $flag): void { if ($flag) { $x = 1; } elseif (true) { $x = 2; } else { echo 'unreachable'; } }
function falseElseif(bool $flag): void { if ($flag) { $x = 1; } elseif (false) { echo 'unreachable'; } else { $x = 2; } }
function allArrays(bool $flag): void { if ($flag) { $a = []; } else { $a = []; } $a[] = 1; }
/** @param \ArrayAccess<int, int> $external */
function maybeArray(bool $flag, \ArrayAccess $external): void { $a = $external; if ($flag) { $a = []; } $a[] = 1; }
/** @param \ArrayAccess<int, int> $external */
function survivingRight(bool $flag, \ArrayAccess $external): void { if ($flag) { $a = $external; return; } else { $a = []; } $a[] = 1; }
/** @param \ArrayAccess<int, int> $external */
function survivingLeft(bool $flag, \ArrayAccess $external): void { if ($flag) { $a = []; } else { $a = $external; return; } $a[] = 1; }
function arrayBeforeBranch(bool $flag): void { $a = []; if ($flag) { $x = 1; } $a[] = 1; }
function arrayCondition(bool $flag): void { if ($a = []) { $x = $flag; } $a[] = 1; }
function ordinary(): void { $x = 1; }
function ordinaryEffect(): void { echo 'effect'; }

/**
 * @pure
 * @throws void
 */
function condition(): bool { return true; }
function pureCondition(): void { if (condition()) { $x = 1; } }
/** @throws void */
function impureCondition(): bool { echo 'effect'; return true; }
function impureCallCondition(): void { if (impureCondition()) { $x = 1; } }
function unknownCondition(string $text): void { if (strlen($text)) { $x = 1; } }
function reachableUnknown(bool $flag): void { if ($flag) { strlen('text'); } }

class PrivateBranch {
    private function local(bool $flag): void { if ($flag) { $x = 1; } else { $x = 2; } }
    public function invoke(bool $flag): void { $this->local($flag); }
}
