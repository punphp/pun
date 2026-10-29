<?php
namespace N;
use Illuminate\Support\Facades\App;
$a = new class {
    public function go() { return App::get('y'); }
};
-----
<?php
namespace N;
use Illuminate\Support\Facades\App;
$a = new class {
    public function __construct(private \Illuminate\Foundation\Application $application)
    {
    }
    public function go() { return $this->application->get('y'); }
};
