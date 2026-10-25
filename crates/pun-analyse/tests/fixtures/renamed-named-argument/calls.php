<?php
namespace RenamedNamedArgument;

function calls(Contract $contract): void {
    $contract->run(value: 'value');
    $contract->run(
        second: 1,
        value: 'value',
    );
    $contract->run('positional');
    $contract->run(unknown: 'unknown');
    $contract->run(...['value' => 'unpacked']);
    $contract->run(...[
        'second' => 2,
        'value' => 'multiline unpacked',
    ]);
    $name = 'run';
    $contract->$name(value: 'dynamic name');
    $contract->{'run'}(value: 'literal dynamic name');
    $contract->RUN(value: 'uppercase method');
    $contract?->run(value: 'nullsafe');
    // @phpstan-ignore argument.parameterRenamedInSubtype
    $contract->run(value: 'ignored');
}
