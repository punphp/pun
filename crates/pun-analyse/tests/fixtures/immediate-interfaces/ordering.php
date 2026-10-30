<?php
namespace ImmediateOrdering;

interface ZFirst { public function run(string $z): void; }
interface ALast { public function run(string $a): void; }
class WrittenFirst implements ZFirst, ALast {
    public function run(string $first): void {}
}
class WrittenReverse implements ALast, ZFirst {
    public function run(string $reverse): void {}
}
interface ZLeft extends ZFirst, ALast { public function run(string $left): void; }
interface ARight extends ALast, ZFirst { public function run(string $right): void; }
function calls(ZFirst $z, ALast $a): void {
    $z->run(z: 'WrittenFirst and ZLeft select ZFirst');
    $a->run(a: 'WrittenReverse and ARight select ALast');
}

use ImmediateOrdering\ZFirst as ZImported;
use ImmediateOrdering\ALast as AImported;
class Imported implements ZImported, \ImmediateOrdering\ALast {
    public function run(string $imported): void {}
}
function imported(ZImported $z): void {
    $z->run(z: 'resolve imports without sorting their spellings');
}
/** @no-named-arguments */
interface ZForbidden { public function run(string $old): void; }
interface AAllowed { public function run(string $old): void; }
function anonymous(AAllowed $allowed): void {
    $instance = new class implements ZForbidden, AAllowed {
        public function run(string $anonymous): void {}
    };
    $allowed->run(old: 'the first prototype forbids names, so no alias record is emitted');
}
