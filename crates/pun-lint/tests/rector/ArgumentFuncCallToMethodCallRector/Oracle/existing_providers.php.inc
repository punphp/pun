<?php
namespace N;
use Illuminate\Contracts\View\Factory;
class A
{
    /** @var Factory */
    protected $views;
    public function go() {
        return view('a');
    }
}
class B extends A
{
    public function getFactory(): ?\Illuminate\Contracts\View\Factory {}
    public function go() {
        return view('a') . $this->other();
    }
    public function other(): Factory {}
}
class C
{
    public function __construct(public \Illuminate\Contracts\View\Factory $f) {}
    public function go() { return view('a'); }
}
class D
{
    public function __construct() { $v = view('a'); }
}
-----
<?php
namespace N;
use Illuminate\Contracts\View\Factory;
class A
{
    /** @var Factory */
    protected $views;
    public function go() {
        return $this->views->make('a');
    }
}
class B extends A
{
    public function getFactory(): ?\Illuminate\Contracts\View\Factory {}
    public function go() {
        return $this->getFactory()->make('a') . $this->other();
    }
    public function other(): Factory {}
}
class C
{
    public function __construct(public \Illuminate\Contracts\View\Factory $f) {}
    public function go() { return $this->f->make('a'); }
}
class D
{
    public function __construct(\Illuminate\Contracts\View\Factory $factory) { $v = $factory->make('a'); }
}
