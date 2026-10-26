<?php
final class Removed {}
final class Retained {}
interface A {}
interface C {}

function reintroduce($query, bool $flag): void {
    if ($query instanceof Removed || $query instanceof Retained) {
        return;
    }
    $joined = $flag ? $query : new Removed();
    \PHPStan\dumpType($joined);
}

function distinctObjects($query, bool $flag): void {
    if ($query instanceof A) {
        return;
    }
    if ($query instanceof C) {
        $joined = $flag ? $query : new Removed();
        \PHPStan\dumpType($joined);
    }
}
