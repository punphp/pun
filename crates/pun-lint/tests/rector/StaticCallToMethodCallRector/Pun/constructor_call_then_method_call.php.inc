<?php
// Pun leaves the class alone: Rector passes the dependency to the constructor call as a plain parameter, then reads `$this->application` in go(), which no class declares.
namespace N;
use Nette\Utils\FileSystem;
use Illuminate\Support\Facades\App;
class A
{
    private $a;

    public function __construct(int $x = 1, $y = null)
    {
        $this->a = $x;
        App::make('x');
    }

    public function run() { return FileSystem::write('a', 'b'); }
    public function go() { return App::get('y') . FileSystem::write('c', 'd'); }
}
