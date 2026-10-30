<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    public function go() { return App::get('y'); }
}
-----
<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    /**
     * @var \Illuminate\Foundation\Application
     */
    private $application;
    public function __construct(\Illuminate\Foundation\Application $application)
    {
        $this->application = $application;
    }
    public function go() { return $this->application->get('y'); }
}
