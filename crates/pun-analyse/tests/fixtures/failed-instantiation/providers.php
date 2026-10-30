<?php
declare(strict_types=1);

namespace FailedInstantiationProviders;

final class Provided {
    public function __construct(int $first, int $second) {}
}
abstract class IllegalProvided {
    public function __construct(int $first, int $second) {}
}
final class OverriddenNever {
    /** @return never */
    public function __construct() { exit; }
}
final class ImplicitBottom { public function __construct() {} }
final class ExplicitBottom { public function __construct() {} }
final class NonacceptingBottom { public function __construct() {} }
final class EmptyProvided {}

function success(): void {
    $value = new Provided($written = 1, $written = 2);
    /*probe*/ $value;
    /*probe*/ $written;
    'after-provider';
}
function illegal(): void {
    $value = new IllegalProvided($written = 3, $written = 4);
    /*probe*/ $value;
    /*probe*/ $written;
    'after-illegal-provider';
}
function overriddenNever(): void {
    $value = new OverriddenNever();
    /*probe*/ $value;
    'after-overridden-never';
}
function implicitBottom(): void {
    $value = new ImplicitBottom();
    /*probe*/ $value;
    'after-implicit-bottom';
}
function explicitBottom(): void {
    $value = new ExplicitBottom();
    /*probe*/ $value;
    'after-explicit-bottom';
}
function nonacceptingBottom(): void {
    $value = new NonacceptingBottom();
    /*probe*/ $value;
    'after-nonaccepting-bottom';
}
function dynamic(): void {
    $target = Provided::class;
    $value = new $target($written = 5, $written = 6);
    /*probe*/ $value;
    /*probe*/ $written;
    'after-dynamic-provider';
}
function dummyConstructor(): void {
    $value = new EmptyProvided($written = 7);
    /*probe*/ $value;
    /*probe*/ $written;
    'after-dummy-provider';
}

final class NoArgumentProvided {}
function noParentheses(): void {
    $value = new NoArgumentProvided;
    /*probe*/ $value;
    'after-no-parentheses-provider';
}

class DeclaringConstructor { public function __construct(int $value) {} }
final class InheritedProvided extends DeclaringConstructor {}
function inheritedConstructor(): void {
    $value = new InheritedProvided($written = 8);
    /*probe*/ $value;
    /*probe*/ $written;
    'after-inherited-provider';
}
