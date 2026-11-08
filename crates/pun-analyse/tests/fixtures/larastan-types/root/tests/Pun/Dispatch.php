<?php

namespace Pun\Dispatch;

use Illuminate\Foundation\Bus\Dispatchable;
use Illuminate\Foundation\Events\Dispatchable as EventDispatchable;

class Job
{
    use Dispatchable;

    public function __construct($a, $b = null)
    {
    }

    public static function fromInside()
    {
        self::dispatch();
        static::dispatch();
    }
}

class ChildJob extends Job
{
    public static function fromChild()
    {
        parent::dispatch();
    }
}

class Variadic
{
    use Dispatchable;

    public function __construct($a, ...$rest)
    {
    }
}

class Exact
{
    use Dispatchable;

    public function __construct($a)
    {
    }
}

class Event
{
    use EventDispatchable;

    public function __construct($a)
    {
    }
}

class Bare
{
    use Dispatchable;
}

class Plain
{
    public static function dispatch()
    {
    }
}

function calls($m, $class, bool $flag, array $args)
{
    Job::dispatch();
    Job::dispatch(1);
    Job::dispatch(a: 1);
    Job::dispatch(...$args);
    Job::dispatch(...[]);
    Job::$m();
    $class::dispatch();
    Plain::dispatch();
    ChildJob::dispatch();
    Variadic::dispatch();
    Exact::dispatch();
    Event::dispatch();
    Event::dispatchSync();
    Job::dispatchIf();
    Job::dispatchIf('yes');
    Job::dispatchIf($flag);
    Job::dispatchUnless(false, 1);
    Job::dispatchIf(boolean: true);
    Bare::dispatchIf(true);
    Bare::dispatchIf(true, 1);
    Bare::dispatch(1);
    Bare::dispatch();
}
