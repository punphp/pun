<?php
/** @param never $never */
function implicitNever($never): void {
    $i = 0;
    $never(++$i);
    $i;
}
function invalidDynamic(int $invalid): void {
    $i = 0;
    $invalid(++$i);
    $i;
}
function stop(): never { throw new Exception(); }
function explicitNever(): void {
    stop()();
}
