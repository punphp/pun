<?php
namespace RenamedNamedArgument;

class TraitCaller { use Caller; }
class OtherTraitCaller { use Caller; }
class TraitImplementation implements Contract { use Implementation; }
class AbstractTraitImplementation {
    use AbstractPrototype;
    public function method(string $renamed): void {}
}
class TraitBase { use Body; }
class TraitChild extends TraitBase {
    public function method(string $renamed): void {}
}
function traitCalls(AbstractTraitImplementation $object, TraitBase $base): void {
    $object->method(renamed: 'not a call on the abstract trait');
    $base->method(original: 'trait declaration belongs to TraitBase');
}
