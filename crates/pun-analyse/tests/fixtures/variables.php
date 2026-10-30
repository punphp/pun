<?php

namespace VariableCertainty;

echo $argc, $argv, $mainUnknown;

function certainty(bool $flag, string $unknown, array $any): void
{
    echo $missing;
    $assigned = $read;
    $assigned = 1;
    echo $assigned;
    $compound += 1;
    if ($flag) {
        $maybe = 1;
    }
    echo $maybe;
    compact('assigned', 'missingCompact', 'maybe', [[['nestedMissing']]], $unknown, $any, 123);
    compact('created', $created = 'created');
    compact($laterName = 'later', $later = 'later');
    isset($issetAllowed, $arrayAllowed[$indexRead]);
    empty($emptyAllowed);
    $coalesceAllowed ?? 'default';
    $coalesceAssign ??= 1;
    echo $coalesceAssign;
    unset($unsetAllowed);
    global $global;
    echo $global;
    $reference =& $referenceAllowed;
    echo $referenceAllowed;
    preg_match('/x/', 'x', $matches);
    echo $matches;
    sort($sortRead);
    takesNative($nativeRead);
    takesNullable($nullableAllowed);
    takesDoc($docAllowed);
    $closure = function () use ($captureRead, &$captureReference) {
        echo $captureRead, $captureReference, $insideMissing, $argc;
    };
    $arrow = fn () => $arrowMissing;
    $name = 'dynamicMissing';
    echo $$name;
    echo ${'indirectMissing'};
    echo $$nameRead;
    isset($$issetNameRead);
    empty($$emptyNameRead);
    $$coalesceNameRead ?? null;
    $chosen = $flag ? 'assigned' : 'otherMissing';
    echo $$chosen;
    $names = ['assigned', ['arrayNameMissing']];
    compact($names);
    $union = $flag ? 'unionMissing' : ['ignoredArrayMissing'];
    compact($union);
    $fn = 'compact';
    $fn('notChecked');
    \COMPACT('upperMissing');
}

function takesNative(int &$value): void {}
function takesNullable(?int &$value): void {}
/** @param int $value */
function takesDoc(&$value): void {}

function cliArguments(): void
{
    echo $argc, $argv;
    global $argc, $argv;
    echo $argc, $argv;
}

function assignedCliArguments(int $argc): void
{
    $argv = [];
    echo $argc, $argv;
}

class Names
{
    public function names(array $array, object $object, \Stringable $stringable, int $int): void
    {
        echo $$array, $$object, $$stringable, $$int, $$this;
        $$array = 1;
    }
}

class References
{
    public function __construct(&$output = null) {}
    public function write(&$output): void {}
    public static function writeStatic(&$output): void {}
    public function readNative(int &$input): void {}
    public static function readStaticNative(int &$input): void {}
    public function next(): self { return $this; }

    public function internal(): void
    {
        self::writeStatic($selfOutput);
        static::writeStatic($staticOutput);
        self::readStaticNative($staticRead);
    }
}

function moreContexts(bool $flag, array $any, References $object): void
{
    new References($constructorOutput);
    $object->write($methodOutput);
    References::writeStatic(output: $staticOutput);
    $class = References::class;
    $class::writeStatic($classOutput);
    $object->readNative($methodRead);
    preg_match(subject: 'x', matches: $namedOutput, pattern: '/x/');
    preg_match('/x/', 'x', $appendOutput[]);
    preg_match('/x/', 'x', ($parenthesizedOutput));
    referencesVariadic($firstOutput, $secondOutput);
    referencesVariadic(second: $namedVariadicOutput);
    isset($objectAllowed->property, $nullsafeAllowed?->property, $staticClassAllowed::$property);
    isset($nullsafeCallAllowed?->method()->property);
    unset($unsetObjectAllowed->property);
    echo $constructorOutput, $methodOutput, $staticOutput, $classOutput, $namedOutput;
    $unary = -$unaryRead;
    $known = 1;
    $knownReference =& $known;
    $selfReference =& $selfReference;
    if ($flag) {
        $maybeReference = 1;
    }
    $reference =& $maybeReference;
    echo $maybeReference;
    $propertyReference =& $object->property;
    compact($flag ? ['one' => 'firstMissing'] : ['two' => 'secondMissing']);
    compact($flag ? ['genericArrayIgnored'] : $any);
    compact($flag ? ['scalarUnionIgnored'] : 123);
}

function referencesVariadic(&...$output): void {}

function nameEffects(): void
{
    echo ${$newName = 'newName'};
    echo $$newName;
}

$arrowAtMain = fn () => $globalCaptureMissing;

function superglobals(): void
{
    echo $GLOBALS, $_SERVER, $_GET, $_POST, $_FILES, $_COOKIE, $_SESSION, $_REQUEST, $_ENV;
    compact('GLOBALS', '_SERVER', '_GET', '_POST', '_FILES', '_COOKIE', '_SESSION', '_REQUEST', '_ENV');
    $name = '_SESSION';
    echo $$name;
    unset($_SESSION);
    echo $_SESSION;
    compact('_SESSION');
    $arrow = fn () => $_SESSION;
    $closure = function (): void {
        echo $_SESSION;
    };
}

/** @var list<int> $annotatedMain */
$annotatedMain;
echo $annotatedMain;
compact('annotatedMain');

function referenceReceiver(): References
{
    return new References();
}

function computedReferenceReceivers(): void
{
    (new References())->write($newOutput);
    referenceReceiver()->write($returnedOutput);
    (new References())->next()->write($chainedOutput);
    referenceReceiver()?->write($nullsafeOutput);
    referenceReceiver()::writeStatic($staticOutput);
    (new References())->readNative($newInput);
    referenceReceiver()->readNative($returnedInput);
    referenceReceiver()?->readNative($nullsafeInput);
    echo $newOutput, $returnedOutput, $chainedOutput, $nullsafeOutput, $staticOutput;
}

function nestedNames(bool $flag): void
{
    echo $${'missingNestedRead'};
    isset($${'missingNestedIsset'});
    empty($${'missingNestedEmpty'});
    $${'missingNestedAssign'} = 1;
    $${'missingNestedCoalesce'} ?? null;
    $${'missingNestedCoalesceAssign'} ??= 1;
    unset($${'missingNestedUnset'});
    if ($flag) {
        $maybeName = 'existingNested';
    }
    $existingNested = 'thirdNested';
    $thirdNested = 1;
    echo $${'maybeName'};
    $knownName = 'existingNested';
    echo $${'knownName'};
    echo $$${'knownName'};
    echo $${($createdName = 'createdName')};
    echo $$${'missingDeep'};
}

class MultilineNames
{
    public function names(\Stringable $castable): void
    {
        echo ${
            $this
        };
        ${
            $this
        } = 1;
        isset(${
            $this
        });
        echo ${
            $castable
        };
    }
}

function negativeOffsetNames(bool $flag): void
{
    $names = [];
    if ($flag) {
        $names['key'] = 'foo';
    } else {
        $names['key'] = 'bar';
        $foo = $bar = 1;
    }
    echo ${$names['key']};
}

function positiveOffsetNames(bool $flag): void
{
    $names = [];
    if ($flag) {
        $names['key'] = 'foo';
        $foo = 1;
    } else {
        $names['key'] = 'bar';
        $bar = 1;
    }
    echo ${($names['key'])};
}

function overwrittenOffsetNames(bool $flag, bool $other): void
{
    $names = [];
    if ($flag) {
        $names['key'] = 'foo';
        $foo = 1;
    } else {
        $names['key'] = 'bar';
        $bar = 1;
    }
    $names['key'] = $other ? 'foo' : 'bar';
    echo ${$names['key']};
}

function integerOffsetNames(bool $flag): void
{
    $names = [];
    if ($flag) {
        $names[0] = 'foo';
    } else {
        $names[0] = 'bar';
        $foo = $bar = 1;
    }
    echo ${$names[0]};
}

function literalOffsetNames(bool $flag): void
{
    if ($flag) {
        $names = ['foo'];
    } else {
        $names = ['bar'];
        $foo = $bar = 1;
    }
    echo ${$names["0"]};
}

function nestedOffsetNames(bool $flag): void
{
    if ($flag) {
        $names = ['nested' => ['key' => 'foo']];
        $foo = 1;
    } else {
        $names = ['nested' => ['key' => 'bar']];
        $bar = 1;
    }
    echo ${$names['nested']['key']};
    $names = ['nested' => ['key' => $flag ? 'bar' : 'foo']];
    echo ${$names['nested']['key']};
}

function variableOffsetName(): void
{
    $names = ['key' => 'untrackedOffset'];
    $key = 'key';
    echo ${$names[$key]};
}

function literalStringOffsetNames(): void
{
    $names = 'foo';
    echo ${$names[0]};
    echo ${$names[-1]};
    echo ${$names[-3]};
    echo ${$names[3]};
    echo ${$names[-4]};
    echo ${$names[9223372036854775807]};
    $names = '';
    echo ${$names[0]};
    echo ${$names[-1]};
}

function invalidStringOffsetNames(): void
{
    $names = 'foo';
    echo ${$names['0']};
    echo ${$names[false]};
    echo ${$names[0.0]};
    echo ${$names[null]};
}

function byteStringOffsetNames(): void
{
    $names = 'éa';
    echo ${$names[2]};
    echo ${$names[-1]};
    $names = "\xffa";
    echo ${$names[1]};
    $names = "\0a";
    echo ${$names[1]};
}

function definedStringOffsetNames(bool $flag): void
{
    $names = 'foo';
    $f = $o = 1;
    echo ${$names[0]};
    echo ${$names[-1]};
    $names = 'bar';
    if ($flag) {
        $b = 1;
    }
    echo ${$names[0]};
}

function replacedStringOffsetNames(): void
{
    $names = 'foo';
    echo ${$names[0]};
    $names = 'bar';
    echo ${$names[0]};
    $index = 0;
    echo ${$names[$index]};
    $index = 1;
    echo ${$names[$index]};
    $names = 'zip';
    echo ${$names[$index]};
}

function writtenStringOffsetNames(): void
{
    $names = 'foo';
    echo ${$names[0]};
    $names[0] = 'b';
    echo ${$names[0]};
}

function unionStringOffsetNames(bool $flag): void
{
    $names = $flag ? 'foo' : 'bar';
    echo ${$names[0]};
}

function missingStringOffsetFallbacks(): void
{
    $names = 'foo';
    echo ${$names[3] ?? 'missingStringOffset'};
    echo ${$names[-4] ?? 'negativeStringOffset'};
    echo ${$names[0] ?? 'unusedStringOffset'};
    $names = '';
    echo ${$names[0] ?? 'emptyStringOffset'};
}

function generalIntegerStringNames(int $index): void
{
    $names = 'foo';
    echo ${$names[$index]};
}

/** @param int<1, 2> $index */
function boundedIntegerStringNames(int $index): void
{
    $names = 'abcd';
    echo ${$names[$index]};
}

/** @param int<0, 1> $index */
function maybeBoundedStringNames(int $index, bool $flag): void
{
    $names = 'fo';
    $f = 1;
    if ($flag) {
        $o = 1;
    }
    echo ${$names[$index]};
}

function cachedFailedOffsetNames(int $index): void
{
    $names = '';
    $copy = $names[$index];
    echo ${$names[$index] ?? 'cachedStringName'};
    echo ${$names[$index]};
    $names = [];
    $key = 'missing';
    $copy = $names[$key];
    echo ${$names[$key] ?? 'cachedArrayName'};
    echo ${$names[$key]};
}

function cachedPostfixOffsetNames(): void
{
    $names = 'foo';
    $index = 4;
    $copy = $names[$index];
    echo ${$names[$index++] ?? 'cachedPostfixName'};
    echo ${$names[$index]};
}
