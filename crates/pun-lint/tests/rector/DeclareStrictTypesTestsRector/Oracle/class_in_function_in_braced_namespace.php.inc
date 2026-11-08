<?php
// c
namespace A {
function f() { class XTest extends \PHPUnit\Framework\TestCase {} }
}
-----
<?php

declare(strict_types=1);

// c
namespace A {
function f() { class XTest extends \PHPUnit\Framework\TestCase {} }
}
