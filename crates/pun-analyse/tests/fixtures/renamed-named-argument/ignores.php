<?php
namespace RenamedNamedArgument;

class IgnoreChild implements Contract {
    // @phpstan-ignore argument.parameterRenamedInSubtype
    public function run(string $fromIgnore, int $second = 0): void {}
}
function ignores(Contract $object, Stable $stable): void {
    // @phpstan-ignore argument.parameterRenamedInSubtype
    $object->run(value: 'ignore just one of several overriding classes');
    // @phpstan-ignore argument.parameterRenamedInSubtype, argument.parameterRenamedInSubtype
    $object->run(value: 'ignore two');
    // @phpstan-ignore-next-line
    $object->run(value: 'ignore all');
    // @phpstan-ignore argument.parameterRenamedInSubtype
    $stable->run(value: 'unused ignore remains unmatched');
    // @phpstan-ignore argument.parameterRenamedInSubtype, argument.parameterRenamedInSubtype
    $stable->run(value: 'both unused ignores remain unmatched');
}
