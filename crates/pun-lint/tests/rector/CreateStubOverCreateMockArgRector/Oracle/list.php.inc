<?php

namespace Pun\Ns2;

final class ListTest extends BaseTest
{
    public function testList(array $pair)
    {
        [, $second] = $pair;
        [$first, , $third] = [$this->createMock(\stdClass::class), 1, 2];
    }
}
-----
<?php

namespace Pun\Ns2;

final class ListTest extends BaseTest
{
    public function testList(array $pair)
    {
        [, $second] = $pair;
        [$first, , $third] = [$this->createStub(\stdClass::class), 1, 2];
    }
}
