<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    public function go($c) { return Thing::make(1) . $c::make(2) . App::get(3); }
}
-----
<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    public function __construct(private \Illuminate\Foundation\Application $application)
    {
    }
    public function go($c) { return Thing::make(1) . $c::make(2) . $this->application->get(3); }
}
