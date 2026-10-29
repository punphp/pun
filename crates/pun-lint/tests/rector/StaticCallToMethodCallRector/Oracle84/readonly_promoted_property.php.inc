<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    public function go() { return App::get('y'); }
}
class B
{
    public function __construct(private int $x)
    {
    }
    public function go() { return App::make($this->x); }
}
-----
<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    public function __construct(private readonly \Illuminate\Foundation\Application $application)
    {
    }
    public function go() { return $this->application->get('y'); }
}
class B
{
    public function __construct(private int $x, private readonly \Illuminate\Foundation\Application $application)
    {
    }
    public function go() { return $this->application->make($this->x); }
}
