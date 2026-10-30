<?php
function nestedCoalesce(array $values): void {
    $values[][] ??= 5;
}
function coalesceIndexRead(array $values, array $index): void {
    $values[$index[]][] ??= 5;
}
function coalesceRightRead(array $values): void {
    $values[][] ??= $values[];
}
function ordinaryReadModifyWrite(array $values): void {
    $values[][] += 5;
}
function ordinaryWrite(array $values): void {
    $values[][] = 5;
}
function referenceRead(array $values): void {
    $reference =& $values[][];
}
function standaloneArrayInList(array $values): void {
    list([$values[]]) = [[1]];
}
