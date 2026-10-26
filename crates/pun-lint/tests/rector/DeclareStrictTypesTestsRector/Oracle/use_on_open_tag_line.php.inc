<?php use X;
class XTest extends \PHPUnit\Framework\TestCase {}


function f() {}
-----
<?php declare(strict_types=1);

use X;
class XTest extends \PHPUnit\Framework\TestCase {}


function f() {}
