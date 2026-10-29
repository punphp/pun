<?php
declare(strict_types=1);

namespace FailedInstantiationControls;

final class Ordinary {
    /** @param-out string $out */
    public function __construct(int &$out) { $out = 'done'; }
}
abstract class AbstractWithConstructor {
    /** @param-out string $out */
    public function __construct(int &$out) { $out = 'done'; }
}
interface InterfaceWithConstructor {
    /** @param-out string $out */
    public function __construct(int &$out);
}
trait TraitWithConstructor {
    /** @param-out string $out */
    public function __construct(int &$out) { $out = 'done'; }
}
final class PrivateConstructor {
    private function __construct() {}
}
final class Stops {
    /** @return never */
    public function __construct() { exit; }
}
abstract class AbstractStops {
    /** @return never */
    public function __construct() { exit; }
}
final class NoParent {
    public function make(): void {
        $value = new parent();
        /*probe*/ $value;
        'after-no-parent';
    }
}
function success(): void {
    $out = 0;
    $value = new Ordinary($out);
    /*probe*/ $value;
    /*probe*/ $out;
    'after-success';
}
function abstractReference(): void {
    $out = 0;
    $value = new AbstractWithConstructor($out);
    /*probe*/ $value;
    /*probe*/ $out;
    'after-abstract-reference';
}
function interfaceReference(): void {
    $out = 0;
    $value = new InterfaceWithConstructor($out);
    /*probe*/ $value;
    /*probe*/ $out;
    'after-interface-reference';
}
function traitReference(): void {
    $out = 0;
    $value = new TraitWithConstructor($out);
    /*probe*/ $value;
    /*probe*/ $out;
    'after-trait-reference';
}
function privateConstructor(): void {
    $value = new PrivateConstructor();
    /*probe*/ $value;
    'after-private';
}
function trueNever(): void {
    $value = new Stops();
    /*probe*/ $value;
    'after-never';
}
function abstractNever(): void {
    $value = new AbstractStops();
    /*probe*/ $value;
    'after-abstract-never';
}
function dynamicNever(): void {
    $target = Stops::class;
    $value = new $target();
    /*probe*/ $value;
    'after-dynamic-never';
}
function argumentNever(): void {
    $value = new Missing(exit);
    /*probe*/ $value;
    'after-argument-never';
}
function acceptsInt(int $value): void {}
function neverArgument(): void {
    acceptsInt(new AbstractStops());
    'after-abstract-never-argument';
}
function objectArgument(): void {
    acceptsInt(new PrivateConstructor());
    'after-object-argument';
}
