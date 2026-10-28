<?php
namespace ImmediateRepeated;

interface Contract { public function run(string $old): void; }
class ParentClass implements Contract {
    public function run(string $middle): void {}
}
class Child extends ParentClass implements Contract {
    public function run(string $new): void {}
}
class Grandchild extends Child implements Contract {
    public function run(string $last): void {}
}
function calls(Contract $contract, ParentClass $parent, Child $child): void {
    $contract->run(old: 'only the parent overrides Contract');
    $parent->run(middle: 'Child overrides ParentClass');
    $child->run(new: 'Grandchild overrides Child');
}
