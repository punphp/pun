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
    public function __construct(
        int $x,
        $y,
        private \Illuminate\Foundation\Application $application
    ) {
        $this->x = $x;
        if ($x) {
            return;
        }
    }
    public function go() { return $this->application->get('y'); }
}
