<?php

namespace GenericPropertyDescriptionQuiet;

/** @template T */
class Box
{
    public int $defaulted = 1;
    /** @var T */
    public $value;
}

/** @param Box<string> $box */
function supplied(Box $box): void
{
    $box->defaulted = 1;
    $box->value = 'right';
}

function bare(Box $box): void
{
    $box->defaulted = 1;
    $box->value = 'right';
}

/** @template T of int|string = string */
class Defaulted
{
    /** @var T */
    public $value;
}

function defaulted(Defaulted $box): void
{
    $box->value = 'right';
}

/** @extends Box<string> */
class Child extends Box {}

function inherited(Child $box): void
{
    $box->defaulted = 1;
    $box->value = 'right';
}

class First
{
    public int $value = 1;
}

class Second
{
    public int $value = 1;
}

/** @param First|Second $box */
function compoundReceiver(First|Second $box): void
{
    $box->value = 1;
}
