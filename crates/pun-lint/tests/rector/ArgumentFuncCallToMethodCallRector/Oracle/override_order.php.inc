<?php
namespace N;
use Illuminate\Contracts\View\Factory;
trait T
{
    public function same(): Factory {}
    public function fromTrait(): Factory {}
}
class P
{
    public function fromTrait() {}
}
class F extends P
{
    use T;
    public function same() {}
    public function go() { return view('a'); }
}
-----
<?php
namespace N;
use Illuminate\Contracts\View\Factory;
trait T
{
    public function same(): Factory {}
    public function fromTrait(): Factory {}
}
class P
{
    public function fromTrait() {}
}
class F extends P
{
    use T;
    public function same() {}
    public function go() { return $this->fromTrait()->make('a'); }
}
