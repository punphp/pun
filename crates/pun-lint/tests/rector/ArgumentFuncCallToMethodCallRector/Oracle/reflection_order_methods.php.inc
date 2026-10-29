<?php
namespace N;
use Illuminate\Contracts\View\Factory;
trait T { public function t(): Factory {} }
class P { public function p(): Factory {} public ?Factory $par; }
class A extends P
{
    use T;
    public function go() { return view('a'); }
}
class B extends P
{
    public Factory $own;
    public function go() { return view('a'); }
}
class C
{
    use T;
    public function z(): Factory {}
    public function go() { return view('a'); }
}
trait TP { public Factory $tp; }
class D
{
    use TP;
    public Factory $own;
    public function go() { return view('a'); }
}
class E
{
    public function __construct() {}
    public function go() { return view('a') . route('x') . config('y'); }
}
-----
<?php
namespace N;
use Illuminate\Contracts\View\Factory;
trait T { public function t(): Factory {} }
class P { public function p(): Factory {} public ?Factory $par; }
class A extends P
{
    use T;
    public function go() { return $this->p()->make('a'); }
}
class B extends P
{
    public Factory $own;
    public function go() { return $this->p()->make('a'); }
}
class C
{
    use T;
    public function z(): Factory {}
    public function go() { return $this->z()->make('a'); }
}
trait TP { public Factory $tp; }
class D
{
    use TP;
    public Factory $own;
    public function go() { return $this->tp->make('a'); }
}
class E
{
    public function __construct(private readonly \Illuminate\Contracts\View\Factory $factory, private readonly \Illuminate\Routing\UrlGenerator $urlGenerator, private readonly \Illuminate\Contracts\Config\Repository $repository) {}
    public function go() { return $this->factory->make('a') . $this->urlGenerator->route('x') . $this->repository->get('y'); }
}
