<?php
namespace RenamedNamedArgument;

use RenamedNamedArgument\Contract as First;
use RenamedNamedArgument\OtherContract as Second;
use function PHPStan\dumpType;

function writtenOrder(First /* native comments are not PHPDoc */ | Second $forward, Second|First $reverse): void {
    $copy = $forward;
    $copy->run(value: 'forward copy');
    $reverse->run(value: 'reverse after a safe union call');
    $forward->run(value: 'forward after two safe calls');
}

function narrowedReceiver(Second|First $object): void {
    assert($object instanceof First);
    $object->run(value: 'do not restore the eliminated union member');
    $object = new Stable();
    $object->run(value: 'do not restore a reassigned receiver');
}

class SourceEffects {
    public function observe(array $value): void {}
    /** @param-out array{a: bool, z: string} $value */
    public function replace(array &$value): void { $value = ['a' => true, 'z' => 'changed']; }
}

/** @param array{z: int, a: bool} $value */
function safeMethod(SourceEffects $effects, array $value): void {
    $copy = $value;
    $effects->observe($copy);
    dumpType($copy);
}

/** @param array{z: int, a: bool} $value */
function referenceMethod(SourceEffects $effects, array $value): void {
    $effects->replace($value);
    dumpType($value);
}

/** @param array{z: int, a: bool} $value */
function reassignmentAfterCall(SourceEffects $effects, array $value): void {
    $effects->observe($value);
    $value = ['a' => false, 'z' => 'new'];
    dumpType($value);
}

/** @param array{z: int, a: bool, ...<never, never>}|string $value */
function narrowedArray(SourceEffects $effects, array|string $value): void {
    assert(is_array($value));
    $effects->observe($value);
    dumpType($value);
}
