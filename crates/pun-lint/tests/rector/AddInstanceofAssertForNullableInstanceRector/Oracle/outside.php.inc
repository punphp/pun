<?php

namespace Pun\Ia;

use PHPUnit\Framework\TestCase;

abstract class AbstractCaseTest extends TestCase
{
    abstract public function testAbstract(Repo $repo): void;
}

final class Outside
{
    public function run(Repo $repo, array $list): void
    {
        foreach ($list as $x) {
            $item = $repo->find();
            $item->name();
        }
    }
}
