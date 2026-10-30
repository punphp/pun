<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    public function __construct(
        int $x,
        $y
    ) {
        $this->x = $x;
        if ($x) {
            return;
        }
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
    public function __construct(
        int $x,
        $y,
        \Illuminate\Foundation\Application $application
    ) {
        $this->x = $x;
        if ($x) {
            return;
        }
        $this->application = $application;
    }
    public function go() { return $this->application->get('y'); }
}
