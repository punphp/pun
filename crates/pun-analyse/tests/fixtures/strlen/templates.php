<?php

namespace StrlenReturns;

use function PHPStan\dumpType;

/**
 * @template I of 123
 * @template R of int<100, 999>
 * @template S of 'abc'
 * @template U of 'a'|'ccc'
 * @template B of true
 * @template F of 1.23
 * @template N of non-empty-string
 * @template T of string
 * @param I $int
 * @param R $range
 * @param S $string
 * @param U $union
 * @param B $bool
 * @param F $float
 * @param N $nonempty
 * @param T $unknown
 */
function templates($int, $range, $string, $union, $bool, $float, $nonempty, $unknown): void
{
    dumpType(strlen($int));
    dumpType(strlen($range));
    dumpType(strlen($string));
    dumpType(strlen($union));
    dumpType(strlen($bool));
    dumpType(strlen($float));
    dumpType(strlen($nonempty));
    dumpType(strlen($unknown));
}

/**
 * @template S of 'abc'
 * @template T of S
 * @template I of 123
 * @template J of I
 * @param T $string
 * @param J $int
 */
function nestedTemplates($string, $int): void
{
    dumpType(strlen($string));
    dumpType(strlen($int));
}

/**
 * @template F of float
 * @template C of class-string
 * @template B of false
 * @template N of null
 * @template S of non-empty-string
 * @template T of S
 * @param F $float
 * @param C $class
 * @param B $bool
 * @param N $null
 * @param T $nestedNonempty
 */
function templateQueries($float, $class, $bool, $null, $nestedNonempty): void
{
    dumpType(strlen($float));
    dumpType(strlen($class));
    dumpType(strlen($bool));
    dumpType(strlen($null));
    dumpType(strlen($nestedNonempty));
}
