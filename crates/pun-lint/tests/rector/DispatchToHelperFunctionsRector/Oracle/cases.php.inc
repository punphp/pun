<?php

namespace App;

use Illuminate\Foundation\Bus\Dispatchable;
use Illuminate\Foundation\Events\Dispatchable as EventDispatchable;

trait Wrapper
{
    use Dispatchable;
}

class Job
{
    use Dispatchable;

    public function __construct(public int $a = 0, public int $b = 0)
    {
    }

    public function again()
    {
        static::dispatch(1);
        self::DISPATCHSYNC();
    }
}

class ChildJob extends Job
{
    public function more()
    {
        parent::dispatch(b: 2);
    }
}

class WrappedJob
{
    use Wrapper;
}

class Event
{
    use EventDispatchable;
}

class Plain
{
    public static function dispatch()
    {
    }
}

Job::dispatch(1, 2);
ChildJob::dispatch(1);
WrappedJob::dispatch();
Event::dispatchSync();
Event::Dispatch();
Plain::dispatch();
Missing::dispatch();
Job::dispatchNow();
$job = 'App\Job';
$job::dispatch();
-----
<?php

namespace App;

use Illuminate\Foundation\Bus\Dispatchable;
use Illuminate\Foundation\Events\Dispatchable as EventDispatchable;

trait Wrapper
{
    use Dispatchable;
}

class Job
{
    use Dispatchable;

    public function __construct(public int $a = 0, public int $b = 0)
    {
    }

    public function again()
    {
        dispatch(new static(1));
        dispatch_sync(new self());
    }
}

class ChildJob extends Job
{
    public function more()
    {
        dispatch(new parent(b: 2));
    }
}

class WrappedJob
{
    use Wrapper;
}

class Event
{
    use EventDispatchable;
}

class Plain
{
    public static function dispatch()
    {
    }
}

dispatch(new \App\Job(1, 2));
dispatch(new \App\ChildJob(1));
dispatch(new \App\WrappedJob());
event(new \App\Event());
event(new \App\Event());
Plain::dispatch();
Missing::dispatch();
Job::dispatchNow();
$job = 'App\Job';
$job::dispatch();
