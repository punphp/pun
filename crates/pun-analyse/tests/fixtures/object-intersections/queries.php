<?php
interface A {}
interface B {}
/** @template T */
interface Box { public function ready(): bool; }
/** @phpstan-assert Box<int> $value */
function ensureBox($value): void {}
function generic($query): void {
    if ($query instanceof A) {
        return;
    }
    ensureBox($query);
    \PHPStan\dumpType($query);
    \PHPStan\dumpType($query->ready());
    \PHPStan\dumpType(is_object($query));
    \PHPStan\dumpType(is_string($query));
    \PHPStan\dumpType((bool) $query);
    $copy = $query;
    \PHPStan\dumpType($copy);
    if ($query instanceof B) {
        \PHPStan\dumpType($query);
    } else {
        \PHPStan\dumpType($query);
    }
}
class Receiver {
    /** @phpstan-assert static $value */
    public function ensureStatic($value): void {}
    /** @phpstan-assert $this $value */
    public function ensureThis($value): void {}
    public function inspectStatic($query): void {
        if ($query instanceof A) {
            return;
        }
        $this->ensureStatic($query);
        \PHPStan\dumpType($query);
    }
    public function inspectThis($query): void {
        if ($query instanceof A) {
            return;
        }
        $this->ensureThis($query);
        \PHPStan\dumpType($query);
    }
}
