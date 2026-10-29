<?php
namespace N;
use Illuminate\Support\Facades\App;
class P
{
    public function __construct($negNeg = - -1, $plusPlus = + +1)
    {
    }
}
class A extends P
{
    public function go() { return App::get(1); }
}
-----
<?php
namespace N;
use Illuminate\Support\Facades\App;
class P
{
    public function __construct($negNeg = - -1, $plusPlus = + +1)
    {
    }
}
class A extends P
{
    public function __construct($negNeg = -(-1), $plusPlus = +(+1), private \Illuminate\Foundation\Application $application)
    {
        parent::__construct($negNeg, $plusPlus);
    }
    public function go() { return $this->application->get(1); }
}
