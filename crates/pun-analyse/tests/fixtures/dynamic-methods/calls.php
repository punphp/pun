<?php declare(strict_types=1);

namespace DynamicMethods;

class Target {
    public function integer(int $value): void {}
    public function text(string $value): void {}
    public function sink(): void {}
    public function number(): int { return 1; }
    public function reference(int &$value): void {}
    /** @no-named-arguments */
    public function noNames(int $value): void {}
    /** @template T
     * @param T $value
     * @param T $other
     * @return T
     */
    public function generic($value, $other) { return $value; }

    public function selfCalls(): void {
        $name = 'integer';
        $this->$name();
        $this->integer();
        $name = 'sink';
        $result = $this->$name();
        $result = $this->sink();
    }
}

function correlated(Target $target, bool $flag): void {
    if ($flag) {
        $name = 'integer';
        $value = 1;
    } else {
        $name = 'text';
        $value = 'ok';
    }
    $target->$name($value);
    $target->{$name}(...[$value]);
    $target->$name(...[$value => $value]);
}

function asymmetric(Target $target, bool $flag): void {
    if ($flag) {
        $name = 'integer';
        $value = 'wrong';
        $arguments = ['value' => 'wrong'];
    } else {
        $name = 'text';
        $value = 'ok';
        $arguments = ['value' => 'ok'];
    }
    $target->$name($value);
    $target->$name(...$arguments);
    $target->$name(value: $value);
}

class Left { public function integer(int $value): void {} }
class Right { public function text(string $value): void {} }
function receivers(Left $left, Right $right, bool $flag): void {
    if ($flag) {
        $name = 'integer';
        $receiver = $left;
        $value = 'bad';
    } else {
        $name = 'text';
        $receiver = $right;
        $value = 'ok';
    }
    $receiver->$name($value);
}

/** @param 'integer'|int $name */
function unionVeto(Target $target, $name): void { $target->$name(false); }
/** @template T of 'integer'|'text'
 * @param T $name
 */
function templateNames(Target $target, string $name): void { $target->$name(false); }
function unknownName(Target $target, string $name): void { $target->$name(false); }

function voids(Target $target, bool $flag): void {
    $name = 'sink';
    $normal = $target->sink();
    $dynamic = $target->$name();
    $expression = $target->{'sink'}();
    $target->$name();
    $name = $flag ? 'sink' : 'number';
    $mixed = $target->$name();
    $target->$name();
    $name = 'integer';
    $target
        ->$name();
    $target
        ->{'integer'}(false);
}

function arguments(Target $target): void {
    $name = 'reference';
    $target->$name('bad');
    $name = 'noNames';
    $target->$name(value: 'bad');
    $name = 'integer';
    $target->$name(unknown: 1);
    $target->$name(...false);
    $name = 'generic';
    $target->$name(1, 'ok');
}
