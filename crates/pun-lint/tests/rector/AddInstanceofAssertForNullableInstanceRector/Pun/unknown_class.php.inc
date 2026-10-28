<?php

namespace Pun\Ia;

use PHPUnit\Framework\TestCase;

final class UnknownTest extends TestCase
{
    public function testUnknown(Repo $repo): void
    {
        $m = $repo->findUnknown();
        $m->run();
    }
}
