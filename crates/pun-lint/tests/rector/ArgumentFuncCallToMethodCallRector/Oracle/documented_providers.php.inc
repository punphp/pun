<?php
namespace N;
use Illuminate\Contracts\View\Factory;
trait ProvidesFactory
{
    protected ?\Illuminate\Contracts\View\Factory $traitFactory;
}
class B
{
    use ProvidesFactory;
    public function go() { return view('a'); }
}
class C
{
    /** @return Factory */
    public function make() {}
    public function go() { return view('a'); }
}
abstract class E
{
    abstract public function f(): Factory|\Countable;
    public function go() { return view('a') . session(); }
}
-----
<?php
namespace N;
use Illuminate\Contracts\View\Factory;
trait ProvidesFactory
{
    protected ?\Illuminate\Contracts\View\Factory $traitFactory;
}
class B
{
    use ProvidesFactory;
    public function go() { return $this->traitFactory->make('a'); }
}
class C
{
    /** @return Factory */
    public function make() {}
    public function go() { return $this->make()->make('a'); }
}
abstract class E
{
    public function __construct(private readonly \Illuminate\Contracts\View\Factory $factory, private readonly \Illuminate\Session\SessionManager $sessionManager)
    {
    }
    abstract public function f(): Factory|\Countable;
    public function go() { return $this->factory->make('a') . $this->sessionManager; }
}
