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
    public function go() { return App::get('y'); }
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
    public function go() { return $this->application->get('y'); }
}
