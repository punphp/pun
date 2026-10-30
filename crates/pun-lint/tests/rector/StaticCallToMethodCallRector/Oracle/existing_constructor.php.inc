<?php
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
    public function go() { return FileSystem::write('c', 'd'); }
}
-----
<?php
namespace N;
use Nette\Utils\FileSystem;
use Illuminate\Support\Facades\App;
class A
{
    private $a;

    public function __construct(int $x = 1, $y = null, \Illuminate\Foundation\Application $application, private \Rector\Tests\Transform\Rector\StaticCall\StaticCallToMethodCallRector\Source\TargetFileSystem $targetFileSystem)
    {
        $this->a = $x;
        $application->make('x');
    }

    public function run() { return $this->targetFileSystem->dumpFile('a', 'b'); }
    public function go() { return $this->targetFileSystem->dumpFile('c', 'd'); }
}
