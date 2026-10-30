<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    /** @required */
    public function autowire(int $x)
    {
        echo $x;
    }
    public function go() { return App::get(1) . App::make(2); }
}
-----
<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    private \Illuminate\Foundation\Application $application;
    /** @required */
    public function autowire(int $x, \Illuminate\Foundation\Application $application)
    {
        echo $x;
        $this->application = $application;
    }
    public function go() { return $this->application->get(1) . $this->application->make(2); }
}
