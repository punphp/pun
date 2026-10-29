<?php
declare(strict_types=1);

namespace FailedInstantiation;

interface Contract {}
trait Reusable {}
enum Choice {}
abstract class Base {}

function interfaceCase(): void {
    $written = 0;
    $value = new Contract($written = 1);
    /*probe*/ $value;
    /*probe*/ $written;
    'after-interface';
}
function traitCase(): void {
    $written = 0;
    $value = new Reusable($written = 2);
    /*probe*/ $value;
    /*probe*/ $written;
    'after-trait';
}
function enumCase(): void {
    $written = 0;
    $value = new Choice($written = 3);
    /*probe*/ $value;
    /*probe*/ $written;
    'after-enum';
}
function abstractCase(): void {
    $written = 0;
    $value = new Base($written = 4);
    /*probe*/ $value;
    /*probe*/ $written;
    'after-abstract';
}
function unknownCase(): void {
    $written = 0;
    $value = new Missing($written = 5);
    /*probe*/ $value;
    /*probe*/ $written;
    'after-unknown';
}
function selfCase(): void {
    $written = 0;
    $value = new self($written = 6);
    /*probe*/ $value;
    /*probe*/ $written;
    'after-self';
}
function staticCase(): void {
    $written = 0;
    $value = new static($written = 7);
    /*probe*/ $value;
    /*probe*/ $written;
    'after-static';
}
function parentCase(): void {
    $written = 0;
    $value = new parent($written = 8);
    /*probe*/ $value;
    /*probe*/ $written;
    'after-parent';
}
function integerCase(): void {
    $written = 0;
    $target = 123;
    $value = new $target($written = 9);
    /*probe*/ $value;
    /*probe*/ $written;
    'after-integer';
}
function nullCase(): void {
    $target = null;
    $value = new $target($written = 10);
    /*probe*/ $value;
    /*probe*/ $written;
    'after-null';
}
function booleanCase(): void {
    $target = false;
    $value = new $target($written = 11);
    /*probe*/ $value;
    /*probe*/ $written;
    'after-boolean';
}
