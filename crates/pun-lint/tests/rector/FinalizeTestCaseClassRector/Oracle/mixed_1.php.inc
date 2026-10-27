<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

readonly class ReadonlyTest extends TestCase
{
}

abstract class AbstractTest extends TestCase
{
}

class Plain extends TestCase
{
}

class SomeTestCase extends TestCase
{
}

class NotATest
{
}

class SubTest extends ReadonlyTest
{
}

#[\Foo] #[\Bar]
class AttrTest extends TestCase
{
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final readonly class ReadonlyTest extends TestCase
{
}

abstract class AbstractTest extends TestCase
{
}

class Plain extends TestCase
{
}

class SomeTestCase extends TestCase
{
}

class NotATest
{
}

final class SubTest extends ReadonlyTest
{
}

#[\Foo] #[\Bar]
final class AttrTest extends TestCase
{
}
