<?php declare(strict_types=1);

namespace DynamicMethodGuards;

class Target {}

/** @param 'first'|'second' $name */
function guards(Target $target, Target $other, string $name, bool $flag): void {
    if (method_exists($target, $name)) {
        $target->$name();
        $target->{$name}();
        $other->$name();
        $unrelated = 1;
        $target->$name();
    }
    $target->$name();
    if (method_exists($target, $name)) {
        $name = 'replacement';
        $target->$name();
    }
}

/** @param 'first'|'second' $name */
function receiverWrite(Target $target, string $name): void {
    if (method_exists($target, $name)) {
        $target = new Target();
        $target->$name();
    }
}

/** @param 'first'|'second' $name */
function terminating(Target $target, string $name): void {
    if (!\method_exists($target, $name)) return;
    $target->$name();
    $target->{'missing'}();
}

/** @param 'first'|'second' $name */
function conditionals(Target $target, string $name, bool $flag): void {
    if (method_exists($target, $name) && $flag) {
        $target->$name();
    }
    if (method_exists($target, $name) || $flag) {
        $target->$name();
    }
    if (method_exists($target, $name) === false) {
        $target->$name();
    } else {
        $target->$name();
    }
    if (method_exists($target, $name)) {
        if ($flag) $name = 'replacement';
        $target->$name();
    }
}

class Methods {
    public function first(int $value): void {}
    public function second(string $value): void {}
}

/** @param 'first'|'second' $name */
function existing(Methods $target, string $name): void {
    if (method_exists($target, $name)) {
        $target->$name(false);
    }
}
