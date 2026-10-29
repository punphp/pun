<?php
namespace N;
use Illuminate\Contracts\View\Factory;
class P { public ?Factory $par; private Factory $priv; }
class B extends P
{
    public Factory $own;
    public function go() { return view('a'); }
}
class P2 { private Factory $priv; }
class B2 extends P2
{
    public function go() { return view('a'); }
}
class B3
{
    public function __construct(public Factory $promo) {}
    public Factory $own;
    public function go() { return view('a'); }
}
class P4 { public Factory $same; }
class B4 extends P4
{
    public Factory $own;
    public Factory $same;
    public function go() { return view('a'); }
}
-----
<?php
namespace N;
use Illuminate\Contracts\View\Factory;
class P { public ?Factory $par; private Factory $priv; }
class B extends P
{
    public Factory $own;
    public function go() { return $this->par->make('a'); }
}
class P2 { private Factory $priv; }
class B2 extends P2
{
    public function __construct(private readonly \Illuminate\Contracts\View\Factory $factory)
    {
    }
    public function go() { return $this->factory->make('a'); }
}
class B3
{
    public function __construct(public Factory $promo) {}
    public Factory $own;
    public function go() { return $this->own->make('a'); }
}
class P4 { public Factory $same; }
class B4 extends P4
{
    public Factory $own;
    public Factory $same;
    public function go() { return $this->same->make('a'); }
}
