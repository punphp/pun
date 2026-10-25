<?php

namespace ParentHints;

function bare(parent $value): parent { throw new \RuntimeException(); }
function nullable(?parent $value): ?parent { return $value; }
function union(parent|null $value): parent|null { return $value; }
function wideUnion(parent|int $value): parent|int { return $value; }
function implicitNullable(parent $value = null): void {}
function objectUnion(parent|object $value): parent|object { return $value; }
function wideObjectUnion(parent|int|object $value): parent|int|object { return $value; }
function mixedUnion(parent|mixed $value): parent|mixed { return $value; }

$bare = function (parent $value): parent { throw new \RuntimeException(); };
$nullable = function (?parent $value): ?parent { return $value; };
$union = function (parent|null $value): parent|null { return $value; };
$wideUnion = function (parent|int $value): parent|int { return $value; };
$arrowBare = fn (parent $value): parent => $value;
$arrowNullable = fn (?parent $value): ?parent => $value;
$arrowUnion = fn (parent|null $value): parent|null => $value;

class NoParent
{
    public function bare(parent $value): parent { throw new \RuntimeException(); }
    public function nullable(?parent $value): ?parent { return $value; }
    public function union(parent|null $value): parent|null { return $value; }
    public function wideUnion(parent|int $value): parent|int { return $value; }
    public function implicitNullable(parent $value = null): void {}

    public function closures(): void
    {
        $bare = function (parent $value): parent { throw new \RuntimeException(); };
        $nullable = function (?parent $value): ?parent { return $value; };
        $union = function (parent|null $value): parent|null { return $value; };
        $wideUnion = function (parent|int $value): parent|int { return $value; };
        $arrowBare = fn (parent $value): parent => $value;
        $arrowNullable = fn (?parent $value): ?parent => $value;
        $arrowUnion = fn (parent|null $value): parent|null => $value;
    }
}

class HasMissingParent extends Missing
{
    public function bare(parent $value): parent { throw new \RuntimeException(); }
    public function compound(
        ?PaReNt
        $value,
    ):
        PaReNt|null
    {
        return $value;
    }
}

class Base {}

class HasParent extends Base
{
    public function bare(parent $value): parent { throw new \RuntimeException(); }
    public function nullable(?parent $value): ?parent { return $value; }
    public function union(parent|null $value): parent|null { return $value; }
    public function wideUnion(parent|int $value): parent|int { return $value; }
    public function implicitNullable(parent $value = null): void {}

    public function closures(): void
    {
        $bare = function (parent $value): parent { throw new \RuntimeException(); };
        $nullable = function (?parent $value): ?parent { return $value; };
        $union = function (parent|null $value): parent|null { return $value; };
        $wideUnion = function (parent|int $value): parent|int { return $value; };
        $arrowBare = fn (parent $value): parent => $value;
        $arrowNullable = fn (?parent $value): ?parent => $value;
        $arrowUnion = fn (parent|null $value): parent|null => $value;
    }
}
