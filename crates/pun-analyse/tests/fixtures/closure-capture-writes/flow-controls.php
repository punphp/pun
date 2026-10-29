<?php declare(strict_types=1);

namespace ClosureFlowControls;

/** @throws void */
function sink(mixed $value): void {}
/** @throws void */
function alias(mixed &$value): void {}
/** @throws void */
function halt(): never { exit; }
/** @throws \LogicException */
function throwsLogic(): void {}

function controls(int $value, int $other, bool $flag, array $array, string $name, \stdClass $object, Guard $guard): void
{
    $parentheses = function () use ($value) { $dead = ($value); };
    $cast = function () use ($value) { $dead = (string) $value; };
    $inputs = function () use ($value) { return $copy = $value; };
    $inputCopy = function () use ($value) { $copy = ($intermediate = $value); };
    $inputSink = function () use ($value) { sink($copy = $value); };
    $inputArray = function () use ($value, $other) { return $copy = [$value, $other]; };
    $arrayItem = function () use ($value, $other) { $copy = [$value, $other]; return $copy[0]; };
    $arrayKey = function () use ($value, $other) { $copy = [$value => $other]; return $copy; };
    $arrayKeys = function () use ($value, $other, $name) { $copy = [2 => $value, 'key' => $other, $name => $value]; return $copy; };
    $nestedArray = function () use ($value) { $copy = [[$value]]; };
    $legacy = function () use ($value) { $copy = array($value); };
    $spread = function () use ($value, $array) { $copy = [...$array, $value]; return $copy; };
    $literalOffsets = function () use ($array) { $array[0] = 1; return $array[1]; };
    $nestedOffsets = function () use ($array) { $array[0][1] = 1; return $array[0]; };
    $dynamicOffset = function () use ($array, $name) { $array[$name] = 1; return $array; };
    $unsetOffset = function () use ($array) { unset($array['key']); return $array; };
    $append = function () use ($array) { $array[] = 1; return $array; };
    $propertyWrite = function () use ($object, $value) { $object->value = $value; };
    $objectOffset = function () use ($value) { $object = new \ArrayObject(); $object['key'] = $value; };
    $destructure = function () use ($value, $array) { list($value) = $array; return $value; };
    $referenceList = function () use ($array) { [$alias] = [&$array]; return $alias; };
    $pre = function () use ($value) { ++$value; };
    $post = function () use ($value) { $value++; };
    $postSink = function () use ($value) { return $value++; };
    $preCopy = function () use ($value) { $dead = ++$value; };
    $coalesce = function () use ($value) { $value ??= 1; return $value; };
    $choice = function () use ($flag, $value, $other) { $copy = $flag ? $value : $other; };
    $shortChoice = function () use ($flag, $value) { $copy = $flag ?: $value; };
    $logic = function () use ($flag, $value) { $flag || $value = 1; return $value; };
    $xor = function () use ($flag, $value) { $flag xor $value = 1; return $value; };
    $elseIf = function () use ($flag, $value, $other) { if ($flag) { $value = 1; } elseif ($other) { $value = 2; } else { $value = 3; } return $value; };
    $for = function () use ($value, $flag) { for ($i = 0; $flag; $i++) { $value = 1; break; } return $value; };
    $continue = function () use ($value, $flag) { while ($flag) { $value = 1; continue; } return $value; };
    $outerBreak = function () use ($value, $flag) { while ($flag) { while ($flag) { $value = 1; break 2; } } return $value; };
    $foreach = function () use ($value, $array) { foreach ($array as $value) {} return $value; };
    $foreachKey = function () use ($value, $array) { foreach ($array as $value => $item) {} return $value; };
    $switch = function () use ($value, $flag) { switch ($flag) { case true: $value = 1; break; default: $value = 2; } return $value; };
    $fallthrough = function () use ($value, $flag) { switch ($flag) { case true: echo $value; case false: $value = 1; } return $value; };
    $static = function () use ($value) { static $local = $value; return $local; };
    $global = function () use ($value) { global $value; return $value; };
    $reference = function () use ($value) { alias($value); $value = 1; };
    $referenceOffset = function () use ($array) { alias($array[0]); };
    $referenceExpression = function () use ($value) { alias($value + 1); };
    $arrowReference = function () use ($value) { return fn (&$output) => $output = $value; };
    $deadAfterCall = function () use ($value) { halt(); echo $value; };
    $exit = function () use ($value) { exit; echo $value; };
    $die = function () use ($value) { die; echo $value; };
    $superglobal = function () use ($value) { echo $_GET; };
    $dynamic = function () use ($value, $other) { $name = 'value'; echo $$name; };
    $indirect = function () use ($value, $other) { echo ${'value'}; };
    $dynamicWrite = function () use ($value, $name) { $$name = 1; };
    $compactKeyed = function () use ($value) { return compact(['label' => 'value']); };
    $compactUnpack = function () use ($value, $array) { return compact(...$array); };
    $nestedTry = function () use ($value) { try { try { throw new \LogicException(); } finally { $value = 1; } } catch (\LogicException) { echo $value; } };
    $finallyBreak = function () use ($value, $flag) { while ($flag) { try { break; } finally { $value = 1; } } return $value; };
    $finallyContinue = function () use ($value, $flag) { while ($flag) { try { continue; } finally { $value = 1; } } return $value; };
    $unmatchedThrow = function () use ($value) { try { throw new \LogicException(); } catch (\RuntimeException) { echo $value; } finally { $value = 1; } };
    $throwThenRead = function () use ($value) { try { throwsLogic(); } catch (\Throwable) { echo $value; } };
    $byReferenceParameter = function (&$parameter) use ($value) { $parameter = $value; };
    $noWrites = function () use (&$value) {};
    $opaqueReference = function () use (&$value) { goto end; end: return; };
    $match = function () use ($value, $flag) { match ($flag) { true => $value = 1, default => $value = 2 }; return $value; };
    $matchRead = function () use ($value, $flag) { match ($flag) { true => $value = 1, default => $value }; };
    $matchCopy = function () use ($value, $flag) { $dead = match ($flag) { true => $value, default => 1 }; };
    $matchNoDefault = function () use ($value, $flag) { match ($flag) { true => $value = 1 }; return $value; };
    $matchConditions = function () use ($value, $other, $flag) { match ($flag) { (bool) $other, true => $value = 1, default => $value = 2 }; return $value; };
    $matchCatch = function () use ($value, $flag) { try { match ($flag) { true => $value = 1 }; } catch (\UnhandledMatchError) { echo $value; } };
    $infiniteWhile = function () use ($value) { while (true) { $value = 1; break; } return $value; };
    $falseWhile = function () use ($value) { while (false) { echo $value; } };
    $infiniteFor = function () use ($value) { for (;;) { $value = 1; break; } return $value; };
    $appendInput = function () use ($value, $array) { $array[] = $value; };
    $nestedAppend = function () use ($value, $array) { $array[][0] = $value; };
    $objectAppend = function () use ($value) { $object = new \ArrayObject(); $object[] = $value; };
    $listKey = function () use ($name, $array, $value) { [$name => $value] = $array; return $value; };
    $listReference = function () use ($value, $array) { [&$value] = $array; };
    $foreachReference = function () use ($value, $array) { foreach ($array as &$value) {} };
    $returnedReference = function &() use ($value) { $value = 1; return $value; };
    $emptyCompact = function () use ($value) { return compact([]); };
    $nestedCompact = function () use ($value) { return compact([['value']]); };
    $unknownCompact = function () use ($value) { return compact([1]); };
    $nullsafe = function () use ($value, $object) { $object?->sink($value); };
    $dynamicProperty = function () use ($value, $object, $name) { $object->{$name} = $value; };
    $staticProperty = function () use ($value, $name) { $name::$value = 1; };
    $classType = function () use ($value, $name) { $name::${$value} = 1; };
    $newArguments = function () use ($value) { new \ArrayObject([$value]); };
    $anonymousClass = function () use ($value) { new class($value) { public function __construct($value) {} }; };
    $guardCopy = function () use ($guard) { $dead = $guard; };
    $guardRelease = function () use ($guard) { $guard = new Guard(); };
    $postCopy = function () use ($value) { $dead = $value++; };
    $nestedInputs = function () use ($value) { return $outer = ($inner = $value++); };
    $literalCopy = function () use ($value) { $literal = [$value]; $copy = $literal[0]; return $copy; };
    $literalDeadCopy = function () use ($value) { $literal = [$value]; $copy = $literal[0]; };
    $offsetDimension = function () use ($value, $array) { $dead = $array[$value]; };
    $nestedOffsetCopy = function () use ($value, $array) { $dead = $array[$value][0]; };
    $skippedMatch = function () use ($value) { match (true) { true => 1, default => $value }; };
    $emptyNested = function () use ($value) { $nested = function ($parameter) { return $parameter; }; };
    $propertyAppend = function () use ($value, $object) { $object->items[] = $value; };
    $referenceAppend = function () use ($array) { $reference =& $array[]; };
    $dynamicGlobal = function () use ($name, $value) { global $$name; };
    $outerContinue = function () use ($value, $flag) { while ($flag) { while ($flag) { $value = 1; continue 2; } } return $value; };
    $unreachableArm = function () use ($value) { match (true) { true => 1, false => $value }; };
    $falseArm = function () use ($value) { match (true) { false => $value, default => 1 }; };
    $indirectGlobal = function () use ($name, $value) { global ${$name}; };
    $returnedAliasCopy = function &() use ($value) { $alias = $value; $alias = 1; return $alias; };
    $returnedValueCopy = function () use ($value) { $alias = $value; $alias = 1; return $alias; };
    $skippedNestedBody = function () use (&$value) { try {} catch (\LogicException) { $nested = function () { $value = 1; }; } };
    $skippedCapture = function () use (&$value) { try {} catch (\LogicException) { $nested = function () use ($value) {}; } };
    $skippedStaticBody = function () use (&$value) { try {} catch (\LogicException) { $nested = static function () { $value = 1; }; } };
    $skippedDynamic = function () use ($value, $name) { try {} catch (\LogicException) { echo $$name; } };
    $skippedIndirect = function () use ($value, $name) { try {} catch (\LogicException) { echo ${$name}; } };
    $skippedCompact = function () use ($value) { try {} catch (\LogicException) { \CoMpAcT('value'); } };
    $skippedExtract = function () use ($value) { try {} catch (\LogicException) { extract([]); } };
    $skippedDefined = function () use ($value) { try {} catch (\LogicException) { get_defined_vars(); } };
    $skippedEval = function () use ($value) { try {} catch (\LogicException) { eval('echo $value;'); } };
    $skippedInclude = function () use ($value) { try {} catch (\LogicException) { include 'missing.php'; } };
    $skippedOrdinaryCall = function () use (&$value) { try {} catch (\LogicException) { sink($value); } };
    $skippedDynamicCall = function () use (&$value, &$name) { try {} catch (\LogicException) { $name($value); } };
    $numericKey = function () use ($value) { $copy = ['0' => $value]; return $copy[0]; };
    $numericRead = function () use ($value) { $copy = [$value]; return $copy['0']; };
    $booleanKey = function () use ($value) { $copy = [true => $value]; return $copy[1]; };
    $nullKey = function () use ($value) { $copy = [null => $value]; return $copy['']; };
    $floatKey = function () use ($value) { $copy = [1.9 => $value]; return $copy[1]; };
    $positiveFloatWrap = function () use ($value) { $copy = [1e19 => $value]; return $copy[-8446744073709551616]; };
    $negativeFloatWrap = function () use ($value) { $copy = [-1e19 => $value]; return $copy[8446744073709551616]; };
    $infiniteKey = function () use ($value) { $copy = [INF => $value]; return $copy[0]; };
    $nanKey = function () use ($value) { $copy = [NAN => $value]; return $copy[0]; };
    $equalKeyUnion = function () use ($value, $flag) { $key = $flag ? false : 0; $copy = [$key => $value]; return $copy[0]; };
    $unknownBooleanKey = function () use ($value, $flag) { $copy = [$flag => $value]; return $copy[0]; };
    $distinctKeyUnion = function () use ($value, $flag) { $key = $flag ? 0 : 1; $copy = [$key => $value]; return $copy[0]; };
}

final class Guard { public function __destruct() {} }
