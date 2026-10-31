<?php
namespace App;
use PHPUnit\Framework\TestCase;
class Obj { }
final class T extends TestCase
{
    public function test(Obj $o)
    {
        $this->assertSame(['my-key' => 1], (array) $o);
    }
}
