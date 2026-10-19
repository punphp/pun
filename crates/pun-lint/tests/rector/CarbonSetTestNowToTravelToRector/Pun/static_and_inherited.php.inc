<?php

namespace App;

use Carbon\Carbon;
use Carbon\CarbonImmutable;
use Illuminate\Foundation\Testing\TestCase;

abstract class Base extends TestCase
{
}

class ViaBase extends Base
{
    public function test()
    {
        Carbon::setTestNow('2024-01-01');
        CarbonImmutable::SETTESTNOW();
        Carbon::now();
        $this->assertTrue(true, static::class);
        static fn () => Carbon::setTestNow();
        static function () {
            Carbon::setTestNow();
        };
    }

    public static function setUpBeforeClass(): void
    {
        Carbon::setTestNow();
    }
}

class NotATest
{
    public function test()
    {
        Carbon::setTestNow('2024-01-01');
    }
}

Carbon::setTestNow('2024-01-01');
-----
<?php

namespace App;

use Carbon\Carbon;
use Carbon\CarbonImmutable;
use Illuminate\Foundation\Testing\TestCase;

abstract class Base extends TestCase
{
}

class ViaBase extends Base
{
    public function test()
    {
        $this->travelTo('2024-01-01');
        $this->travelTo(null);
        Carbon::now();
        $this->assertTrue(true, static::class);
        static fn () => Carbon::setTestNow();
        static function () {
            Carbon::setTestNow();
        };
    }

    public static function setUpBeforeClass(): void
    {
        Carbon::setTestNow();
    }
}

class NotATest
{
    public function test()
    {
        Carbon::setTestNow('2024-01-01');
    }
}

Carbon::setTestNow('2024-01-01');
