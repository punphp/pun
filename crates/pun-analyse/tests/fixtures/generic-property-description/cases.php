<?php

namespace GenericPropertyDescription;

/** @template T */
class Box
{
    public int $defaulted = 1;
    /** @var T */
    public $value;

    public function onThis(): void
    {
        $this->defaulted = 'wrong';
    }
}

function bare(Box $box): void
{
    $box->defaulted = 'wrong';
}

/** @param Box<int> $box */
function supplied(Box $box): void
{
    $box->defaulted = 'wrong';
    $box->value = 'wrong';
    $box->value = 1;
}

/** @template T of int|string = string */
class Defaulted
{
    public int $marker = 1;
    /** @var T */
    public $value;
}

function defaults(Defaulted $box): void
{
    $box->marker = 'wrong';
    $box->value = 1;
    $box->value = 'right';
}

/**
 * @template Left
 * @template Right
 */
class PairBox
{
    public int $marker = 1;
    /** @var Left */
    public $left;
    /** @var Right */
    public $right;
}

/**
 * @template First
 * @template Second
 * @extends PairBox<Second, First>
 */
class Swapped extends PairBox {}

/** @extends Swapped<string, int> */
class Concrete extends Swapped {}

/** @param Swapped<string, int> $box */
function inherited(Swapped $box): void
{
    $box->marker = 'wrong';
    $box->left = 'wrong';
    $box->right = 1;
    $box->left = 1;
    $box->right = 'right';
}

function transitive(Concrete $box): void
{
    $box->marker = 'wrong';
    $box->left = 'wrong';
    $box->right = 1;
}

/**
 * @template T of int|string
 * @param Box<T> $box
 */
function template(Box $box): void
{
    $box->defaulted = 'wrong';
    $box->value = false;
}

class Plain
{
    public int $defaulted = 1;
}

function plain(Plain $box): void
{
    $box->defaulted = 'wrong';
}

/** @template T of int */
class Bounded
{
    public string $marker = '';
    /** @var T */
    public $value;
}

function bounded(Bounded $box): void
{
    $box->marker = 1;
    $box->value = 'wrong';
}

class InheritedDefault extends Defaulted {}

function inheritedDefault(InheritedDefault $box): void
{
    $box->marker = 'wrong';
    $box->value = 1;
}
