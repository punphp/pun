<?php declare(strict_types=1);

namespace DynamicNullsafeVoids;

class Target {
    public function sink(): void {}
    public function number(): int { return 1; }
    public function nothing(): null { return null; }
    /** @return void|null */
    public function nullableVoid() {}
    /** @return ($flag is true ? void : null) */
    public function conditional(bool $flag) {}
}

function calls(?Target $target, bool $flag): void {
    $literal = $target?->sink();
    $target?->sink();
    $name = 'sink';
    $dynamic = $target?->$name();
    $name = $flag ? 'sink' : 'number';
    $mixed = $target?->$name();
    $name = $flag ? 'sink' : 'nothing';
    $nullable = $target?->$name();
    $declared = $target?->nullableVoid();
    $conditional = $target?->conditional($flag);
    $alwaysVoid = $target?->conditional(true);
    $alwaysNull = $target?->conditional(false);
    $target?->sink($target?->sink());
}

function skipped(): void {
    $target = null;
    $value = $target?->sink();
}

function nested(?Target $target, Target $nonNullable, ?Target $other): void {
    $target?->sink($target?->sink());
    $target?->sink($nonNullable?->sink());
    $target?->sink($other?->sink());
    $target?->sink(($target = new Target())?->sink());
}
