<?php
namespace ImmediateTransitive;

interface AncestorContract { public function run(string $old): void; }
interface ZNarrow extends AncestorContract { public function run(string $middle): void; }
interface EmptyNarrow extends ZNarrow {}
class AncestorFirst implements AncestorContract, ZNarrow {
    public function run(string $first): void {}
}
class DescendantFirst implements ZNarrow, AncestorContract {
    public function run(string $second): void {}
}
class Indirect implements AncestorContract, EmptyNarrow {
    public function run(string $indirect): void {}
}
function calls(AncestorContract $ancestor, ZNarrow $narrow): void {
    $ancestor->run(old: 'only ZNarrow overrides the ancestor');
    $narrow->run(middle: 'all three implementations override ZNarrow');
}
