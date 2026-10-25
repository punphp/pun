<?php

namespace UnsetWriteBack;

function scalarRoot(): void
{
    $value = 3;
    $before = $value;
    unset($value['bad']);
    \PHPStan\dumpType($value);
    $after = $value;
    \PHPStan\dumpType($after);
    \PHPStan\dumpType($before);
    unset($value['again']);
    $value = ['restored' => 1];
    unset($value['restored']);
    \PHPStan\dumpType($value);
}

function nestedAndSiblings(): void
{
    $array = ['branch' => ['leaf' => 1], 'sibling' => ['safe' => 2]];
    $copy = $array;
    unset($array['branch']['leaf']['bad']);
    \PHPStan\dumpType($array);
    \PHPStan\dumpType($copy);
    \PHPStan\dumpType($array['sibling']);
    unset($array['branch']['leaf']['again']);
    $array['branch']['leaf'] = ['new' => 4];
    unset($array['branch']['leaf']['new']);
    \PHPStan\dumpType($array);
}

function validNestedMutation(): void
{
    $array = ['branch' => ['leaf' => 1, 'kept' => 2], 'sibling' => 3];
    unset($array['branch']['leaf']);
    \PHPStan\dumpType($array);
    unset($array['branch']['leaf']);
    \PHPStan\dumpType($array);
}

function nullIsNotPromoted(): void
{
    $null = null;
    unset($null['x']);
    \PHPStan\dumpType($null);
    $array = ['branch' => null, 'sibling' => 2];
    unset($array['branch']['x']);
    \PHPStan\dumpType($array);
    \PHPStan\dumpType($array['branch']);
}

/** @param 'branch' $key */
function cachedOffsets(string $key): void
{
    $array = ['branch' => ['leaf' => 1], 'sibling' => ['safe' => 2]];
    $before = $array[$key]['leaf'];
    \PHPStan\dumpType($array[$key]);
    unset($array['branch']['leaf']['bad']);
    \PHPStan\dumpType($array[$key]);
    \PHPStan\dumpType($before);
    \PHPStan\dumpType($array['sibling']['safe']);
    unset($array[$key]['leaf']['again']);
}

function oneIndexEvaluation(): void
{
    $index = 0;
    $array = [1, 2];
    unset($array[$index++]['bad']);
    \PHPStan\dumpType($index);
    \PHPStan\dumpType($array);
}

/** @param 'a'|'b' $key */
function weakUnionKeys(string $key): void
{
    $array = ['a' => ['leaf' => 1, 'keptA' => 3], 'b' => ['leaf' => 2, 'keptB' => 4]];
    unset($array[$key]['leaf']);
    \PHPStan\dumpType($array);
    \PHPStan\dumpType($array['a']);
    \PHPStan\dumpType($array['b']);
}

/** @param 'a'|'b' $key */
function weakInvalidUnionKeys(string $key): void
{
    $array = ['a' => 1, 'b' => 2, 'sibling' => 3];
    unset($array[$key]['bad']);
    \PHPStan\dumpType($array);
    \PHPStan\dumpType($array['sibling']);
}

function multipleTargets(): void
{
    $array = ['a' => ['leaf' => 1], 'sibling' => 2];
    unset($array['a']['leaf']['bad'], $array['a']['leaf']['again']);
    \PHPStan\dumpType($array);
}

function scalarKinds(): void
{
    $bool = false;
    $float = 1.5;
    $string = 'a';
    unset($bool[0], $float[0], $string[0]);
    \PHPStan\dumpType($bool);
    \PHPStan\dumpType($float);
    \PHPStan\dumpType($string);
}

function readsOfErrorMembers(): void
{
    $array = ['a' => 1];
    unset($array['a']['bad']);
    \PHPStan\dumpType($array);
    \PHPStan\dumpType($array['a']);
    unset($array['a']['again']);
    \PHPStan\dumpType($array);
}

function missingReceiver(): void
{
    $array = ['sibling' => 1];
    unset($array['missing']['bad']);
    \PHPStan\dumpType($array);
    \PHPStan\dumpType($array['sibling']);
}

function invalidContainingReceiver(): void
{
    $scalar = 1;
    unset($scalar['a']['b']);
    \PHPStan\dumpType($scalar);
}

/** @param 'a'|'b' $key */
function narrowedKey(string $key): void
{
    $array = ['a' => ['leaf' => 1], 'b' => ['leaf' => 2]];
    if ($key === 'a') {
        unset($array[$key]['leaf']);
        \PHPStan\dumpType($array);
    }
}

function nullAncestor(): void
{
    $null = null;
    unset($null['a']['b']);
    \PHPStan\dumpType($null);
}

function unknownContainingKey(string $key): void
{
    $array = ['a' => 1, 'b' => 2];
    unset($array[$key]['bad']);
    \PHPStan\dumpType($array);
}

/** @param array{a: int}|null $array */
function nullableParent(?array $array): void
{
    unset($array['a']['bad']);
    \PHPStan\dumpType($array);
}

final class Holder
{
    public array $items = ['branch' => ['leaf' => 1], 'sibling' => 2];

    public function test(): void
    {
        $this->items = ['branch' => ['leaf' => 1], 'sibling' => 2];
        unset($this->items['branch']['leaf']['bad']);
        \PHPStan\dumpType($this->items);
        \PHPStan\dumpType($this->items['sibling']);
    }
}

function absentRoot(): void
{
    unset($missing['key']);
    \PHPStan\dumpType($missing);
    unset($missing);
}

function explicitlyAbsentRoot(): void
{
    $array = [];
    unset($array);
    unset($array['key']);
    \PHPStan\dumpType($array);
    unset($array);
}
