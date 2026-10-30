<?php
function assignmentShort(): void {
    list([$item]) = [[1]];
}
function assignmentLegacy(): void {
    list(array($item)) = [[1]];
}
function localWrite(): void {
    $item = 1;
}
function literalEffect(): void {
    list([print 'value']) = [[1]];
}
function keyEffect(): void {
    list(print 'key' => $item) = [1 => 1];
}
function propertyEffect(stdClass $object): void {
    list($object->item) = [1];
}
function localReference(): void {
    $value = 0;
    list(&$item) = [$value];
}
function byReference(int &$item): void {
    $item = 1;
}
function localOffset(): void {
    $items = [];
    [$items[0]] = [1];
}
