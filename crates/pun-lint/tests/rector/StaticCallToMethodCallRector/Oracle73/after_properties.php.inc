<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    private $x;
    public function go() { return App::get($this->x); }
}
-----
<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    private $x;
    /**
     * @var \Illuminate\Foundation\Application
     */
    private $application;
    public function __construct(\Illuminate\Foundation\Application $application)
    {
        $this->application = $application;
    }
    public function go() { return $this->application->get($this->x); }
}
