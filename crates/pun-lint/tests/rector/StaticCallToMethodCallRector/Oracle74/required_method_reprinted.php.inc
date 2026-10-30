<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    /** @required */
    public function &autowire()
    {
    }
    public function go() { return App::get(1); }
}
class B
{
    /** @required */
    public function autowire(
        // the x
        int $x,
        #[\SensitiveParameter] $y
    ): void {
    }
    public function go() { return App::get(1); }
}
-----
<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    private \Illuminate\Foundation\Application $application;
    /** @required */
    public function &autowire(\Illuminate\Foundation\Application $application)
    {
        $this->application = $application;
    }
    public function go() { return $this->application->get(1); }
}
class B
{
    private \Illuminate\Foundation\Application $application;
    /** @required */
    public function autowire(
        // the x
        int $x,
        #[\SensitiveParameter] $y,
        \Illuminate\Foundation\Application $application
    ): void
    {
        $this->application = $application;
    }
    public function go() { return $this->application->get(1); }
}
