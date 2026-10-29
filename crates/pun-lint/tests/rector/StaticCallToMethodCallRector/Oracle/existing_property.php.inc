<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    private $application;
    public function go() { return App::get('y'); }
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
    public function go() { return $this->application->get('y'); }
}
