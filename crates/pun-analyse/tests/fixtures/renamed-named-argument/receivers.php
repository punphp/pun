<?php
namespace RenamedNamedArgument;

function receivers(
    Contract|OtherContract $union,
    Contract|DifferentContract $different,
    Contract&OtherContract $intersection,
    Contract|string $partial,
    ?Contract $nullable,
    Contract $object,
    mixed $mixed,
    string $unknownName,
    OtherContract|Contract $reversed,
): void {
    $union->run(value: 'union');
    $union->RUN(value: 'uppercase union');
    $different->run(value: 'different union names');
    $intersection->run(value: 'intersection');
    $partial->run(value: 'partial union');
    $nullable->run(value: 'nullable');
    $nullable?->run(value: 'nullsafe');
    $mixed->run(value: 'mixed');
    $object->$unknownName(value: 'unknown method name');
    $name = rand() ? 'run' : 'missing';
    $object->$name(value: 'alternative method names');
    $object->{rand() ? 'run' : 'RUN'}(value: 'case alternatives');
    $reversed->run(value: 'opposite native union declaration order');
}
