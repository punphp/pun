<?php
class nullsafescopeedge {
    public ?self $child = null;
    /** @var array{sink: self|null} */
    public array $holders;
    public function foo($value, $later = null): void {}
    public function inspect($value, $later = null): void {}
}
function nullsafeScopeNestedCalls(?nullsafescopeedge $sink): void {
    $inspect = new nullsafescopeedge();
    $sink?->foo($inspect->inspect($sink)/* observe */, $sink?->foo($born = 1));
    $inspect->inspect($sink, $born); // observe
}
/** @template TSink of (nullsafescopeedge|null)
 * @param TSink $sink
 */
function nullsafeScopeTemplate($sink): void {
    $inspect = new nullsafescopeedge();
    $sink?->foo($born = 2);
    $inspect->inspect($born); // observe
}
function nullsafeScopePropertyArray(nullsafescopeedge $box): void {
    $inspect = new nullsafescopeedge();
    $box->holders['sink']?->foo($box->holders['sink'] = null);
    $inspect->inspect($box->holders, $box->holders['sink']); // observe
}
function nullsafeScopeAssignedSpines(?nullsafescopeedge $maybe): void {
    $inspect = new nullsafescopeedge();
    ($sink = new nullsafescopeedge())->child?->foo($sink->child = null);
    ($holders = ['sink' => $maybe])['sink']?->foo($holders = []);
    $inspect->inspect($sink->child, $holders); // observe
}
function nullsafeScopeNullsafeChain(?nullsafescopeedge $sink): void {
    $inspect = new nullsafescopeedge();
    $sink?->child?->foo($born = 3);
    $inspect->inspect($sink, $born); // observe
}
function nullsafeScopeNonNullMixed(mixed $sink): void {
    if ($sink === null) return;
    $inspect = new nullsafescopeedge();
    $sink?->foo($born = 4);
    $inspect->inspect($born); // observe
}
/** @param array{sink?: nullsafescopeedge|null} $holders */
function nullsafeScopeOptionalOffset(array $holders): void {
    $inspect = new nullsafescopeedge();
    $holders['sink']?->foo($born = 5);
    $inspect->inspect($holders, $born); // observe
}
