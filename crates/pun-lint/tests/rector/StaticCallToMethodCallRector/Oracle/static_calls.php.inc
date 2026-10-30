<?php
namespace N;
use Illuminate\Support\Facades\App;
use Illuminate\Support\Facades\App as Ap;
class A
{
    public function go() {
        \Illuminate\Support\Facades\App::get(
            'a',
            // c
            'b'
        );
        Ap::make(App::get('x'), 2);
        App::{'x'}();
        App::get(...);
        return APP::make(1);
    }
    public static function st() { return App::make(1); }
}
-----
<?php
namespace N;
use Illuminate\Support\Facades\App;
use Illuminate\Support\Facades\App as Ap;
class A
{
    public function __construct(private \Illuminate\Foundation\Application $application)
    {
    }
    public function go() {
        $this->application->get(
            'a',
            // c
            'b'
        );
        $this->application->make($this->application->get('x'), 2);
        App::{'x'}();
        $this->application->get(...);
        return $this->application->make(1);
    }
    public static function st() { return (new \Illuminate\Foundation\Application())->make(1); }
}
