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
