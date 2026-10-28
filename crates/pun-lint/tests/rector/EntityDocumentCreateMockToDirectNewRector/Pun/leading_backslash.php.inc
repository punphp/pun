<?php

namespace Pun\Tests;

final class LeadingTest
{
    public function testLeading()
    {
        $aMock = $this->createMock('\Pun\Entity\Listeners');
    }
}
-----
<?php

namespace Pun\Tests;

final class LeadingTest
{
    public function testLeading()
    {
        $a = new \Pun\Entity\Listeners();
    }
}
