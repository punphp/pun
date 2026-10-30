<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    public function __construct(
        int $x,
        $y,
    ) {}
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
        private \Illuminate\Foundation\Application $application,
    ) {}
    public function go() { return $this->application->get('y'); }
}
