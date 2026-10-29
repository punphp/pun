<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    public $application { set => $value; }
    public function go() { return App::get(1); }
}
-----
<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    public $application { set => $value; }
    public function __construct(\Illuminate\Foundation\Application $application)
    {
        $this->application = $application;
    }
    public function go() { return $this->application->get(1); }
}
