<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    private $application;
    public function go() { return App::get(1) . App::make(2); }
}
-----
<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    private $application;
    public function __construct(\Illuminate\Foundation\Application $application)
    {
        $this->application = $application;
    }
    public function go() { return $this->application->get(1) . $this->application->make(2); }
}
