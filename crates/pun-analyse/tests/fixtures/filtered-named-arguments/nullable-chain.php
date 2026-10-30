<?php
namespace NullableNamedChain;

class Base {
    public function run(int $old): void {}
}
class Child extends Base {
    public function run(int $new): void {}
}
class NonNullRoot {
    public function next(): Base { return new Base(); }
}
class NullableRoot {
    public function next(): ?Base { return rand() ? new Base() : null; }
}
function probe(?NonNullRoot $good, ?NullableRoot $nullable): void {
    $good?->next()->run(old: 1);
    $nullable?->next()->run(old: 1);
    $nullable?->next()?->run(old: 1);
}
