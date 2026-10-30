<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    public function go() {
        $f = function () { return App::make(App::get('x')); };
        return fn() => App::bound('q');
    }
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
    public function go() {
        $f = function () { return $this->application->make($this->application->get('x')); };
        return fn() => $this->application->bound('q');
    }
}
