<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    /** @inject */
    public $application;
    public function go() { return App::get(1); }
}
-----
<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    /** @inject */
    public $application;
    public function go() { return $this->application->get(1); }
}
