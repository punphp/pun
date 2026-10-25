<?php
namespace RenamedNamedArgument;

use function PHPStan\dumpType;

/**
 * @template T of (array{kind: 'string', value: string, ...<string, string>}|array{kind: 'int', value: int, ...<string, int|string>})
 * @param T $value
 */
function narrowedTemplate(Contract $object, $value): void {
    assert(is_string($value['value']));
    $object->run(value: 'safe call before a narrowed template');
    dumpType($value['value']);
    $object->run(...$value);
}
