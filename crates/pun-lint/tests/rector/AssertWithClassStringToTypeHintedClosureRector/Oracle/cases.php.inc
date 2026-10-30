<?php

namespace App;

use Illuminate\Support\Facades\Bus;
use Illuminate\Support\Facades\Event;

class SomeJob
{
}

class AssertTest
{
    /**
     * @param class-string<SomeJob> $class
     */
    public function run(string $class)
    {
        Event::assertDispatched(SomeJob::class, function (object $event) {
            return true;
        });
        Bus::ASSERTDISPATCHED(callback: static function (&$job) {
                return $job !== null;
            }, command: SomeJob::class);
        Bus::assertDispatched($class, (fn ($job = null) => true));
        Bus::assertDispatched('App\SomeJob', fn (SomeJob ...$jobs) => true);
        Bus::assertDispatched(
            SomeJob::class,
            function ($job) {
                if ($job) {
                    return true;
                }
            }
        );
        Bus::assertDispatched(SomeJob::class, fn () => true);
        Bus::assertDispatched(SomeJob::class, 3);
        Bus::assertNotDispatched(SomeJob::class, fn ($job) => true);
    }
}
-----
<?php

namespace App;

use Illuminate\Support\Facades\Bus;
use Illuminate\Support\Facades\Event;

class SomeJob
{
}

class AssertTest
{
    /**
     * @param class-string<SomeJob> $class
     */
    public function run(string $class)
    {
        Event::assertDispatched(function (\App\SomeJob $event) {
            return true;
        });
        Bus::ASSERTDISPATCHED(callback: static function (&$job) {
                return $job !== null;
            }, command: SomeJob::class);
        Bus::assertDispatched(fn (\App\SomeJob $job = null) => true);
        Bus::assertDispatched(fn (\App\SomeJob ...$jobs) => true);
        Bus::assertDispatched(function (\App\SomeJob $job) {
            if ($job) {
                return true;
            }
        });
        Bus::assertDispatched(SomeJob::class, fn () => true);
        Bus::assertDispatched(SomeJob::class, 3);
        Bus::assertNotDispatched(SomeJob::class, fn ($job) => true);
    }
}
