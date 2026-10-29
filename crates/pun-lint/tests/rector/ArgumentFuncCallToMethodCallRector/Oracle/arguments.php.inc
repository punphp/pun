<?php
namespace N;
abstract class A
{
    public function go(array $arr, $mixed, string $s) {
        config($arr);
        config($mixed);
        config(['a' => 1] + []);
        config();
        view(view('x'));
        \view('y');
        View('z');
        return route(...$arr);
    }
    abstract public function ab();
    public static function st() { return view('x'); }
}
-----
<?php
namespace N;
abstract class A
{
    public function __construct(private readonly \Illuminate\Contracts\Config\Repository $repository, private readonly \Illuminate\Contracts\View\Factory $factory, private readonly \Illuminate\Routing\UrlGenerator $urlGenerator)
    {
    }
    public function go(array $arr, $mixed, string $s) {
        $this->repository->set($arr);
        $this->repository->get($mixed);
        $this->repository->set(['a' => 1] + []);
        $this->repository;
        $this->factory->make($this->factory->make('x'));
        $this->factory->make('y');
        $this->factory->make('z');
        return $this->urlGenerator->route(...$arr);
    }
    abstract public function ab();
    public static function st() { return view('x'); }
}
