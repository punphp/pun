<?php

namespace App\Tests;

use App\Event;
use PHPUnit\Framework\TestCase;

final class QTest extends TestCase
{
    public function test()
    {
        $mock->expects($this->once())->WITH(
            $this->callback(fn ($e) => $e instanceof Event),
            $this->callback(function ($e) {
                return $e instanceof Event;
            }),
            $this->callback(function ($e) {
                return $e instanceof self;
            }),
            $this->callback(function ($e) {
                return $e instanceof $other;
            }),
            $this->callback(function ($e) {
                self::assertInstanceOf(Event::class, $e);
                return false;
            }),
            $this->callback(function ($e) {
                // why
                return $e instanceof Event;
            }),
            $other->callback(function ($e) {
                return ($e instanceof Event);
            }),
        );
    }
}
-----
<?php

namespace App\Tests;

use App\Event;
use PHPUnit\Framework\TestCase;

final class QTest extends TestCase
{
    public function test()
    {
        $mock->expects($this->once())->WITH(
            $this->callback(fn ($e) => $e instanceof Event),
            $this->isInstanceOf(\App\Event::class),
            $this->isInstanceOf(self::class),
            $this->callback(function ($e) {
                return $e instanceof $other;
            }),
            $this->callback(function ($e) {
                self::assertInstanceOf(Event::class, $e);
                return false;
            }),
            $this->isInstanceOf(\App\Event::class),
            $this->isInstanceOf(\App\Event::class),
        );
    }
}
