<?php
namespace N;
use Illuminate\Support\Facades\App;
class A extends Unknown
{
    public function go() { return App::get('y'); }
}
class E extends \Exception
{
    public function go() { return App::get('y'); }
}
class F extends \ArrayIterator
{
    public function go() { return App::get('y'); }
}
-----
<?php
namespace N;
use Illuminate\Support\Facades\App;
class A extends Unknown
{
    public function __construct(private \Illuminate\Foundation\Application $application)
    {
    }
    public function go() { return $this->application->get('y'); }
}
class E extends \Exception
{
    public function __construct(private \Illuminate\Foundation\Application $application)
    {
    }
    public function go() { return $this->application->get('y'); }
}
class F extends \ArrayIterator
{
    public function __construct(private \Illuminate\Foundation\Application $application)
    {
    }
    public function go() { return $this->application->get('y'); }
}
