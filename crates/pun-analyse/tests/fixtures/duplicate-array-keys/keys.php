<?php

namespace InferredDuplicateKeys;

/**
 * @param 'a'|'b' $pair
 * @param 'a'|'b'|'c' $three
 */
function pendingUnions(string $pair, string $three): void
{
    $a = [$three => 0, 'a' => 1, 'b' => 2, 'c' => 3];
    $b = [$pair => 0, 'unrelated' => 1, 'a' => 2, 'b' => 3];
    $c = ['a' => 0, 'b' => 1, $pair => 2, $pair => 3];
    $d = [$pair => 0, $pair => 1];
    $e = ['a' => 0, $pair => 1];
    $f = [$three => 0, 'a' => 1];
}

/** @return 'called' */
function key(): string
{
    return 'called';
}

function inferredScalars(bool $boolean, int $unknown, mixed $mixed): void
{
    $integer = -4;
    $float = -4.9;
    $string = '-4';
    $false = false;
    $true = true;
    $null = null;
    $a = [$integer => 0, $float => 1, $string => 2, -4 => 3];
    $b = [$false => 0, 0 => 1, $true => 2, 1 => 3];
    $c = [$null => 0, '' => 1];
    $d = [0 => 0, 1 => 1, $boolean => 2];
    $e = [key() => 0, 'called' => 1];
    $f = [$unknown => 0, 5 => 1, 2, 6 => 3];
    $g = [$mixed => 0, 5 => 1, 2, 6 => 3];
    $h = [$integer => 0, 1, -3 => 2];
    $i = ['0' => 0, $false => 1, '00' => 2, '+0' => 3];
}

/** @param '1'|true $sameKey */
function castBeforeBookkeeping(string|bool $sameKey): void
{
    $a = [$sameKey => 0, 1, 2 => 2];
    $b = [1 => 0, $sameKey => 1];
}

/** @param int|'a' $open */
function openUnion(int|string $open): void
{
    $a = ['a' => 0, $open => 1];
    $b = [$open => 0, 0 => 1, 2, 1 => 3];
}

/**
 * @template T of 'a'|'b'
 * @param T $key
 */
function templateKey(string $key): void
{
    $a = ['a' => 0, 'b' => 1, $key => 2];
}

function wrappingFloatIsNotASaturatingCast(): void
{
    $large = 9.223372036854776E18;
    $a = [$large => 0, 9223372036854775807 => 1];
    $b = [9.223372036854776E18 => 0, 9223372036854775807 => 1];
}
