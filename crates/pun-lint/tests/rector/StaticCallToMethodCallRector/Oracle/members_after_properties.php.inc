<?php
namespace N;
use Nette\Utils\FileSystem;
trait T {}
class A
{
    use T;
    const X = 1;
    private $a;
    public $b;
    const Y = 2;
    /** doc */
    public function run() { return FileSystem::write('a', 'b'); }
}
-----
<?php
namespace N;
use Nette\Utils\FileSystem;
trait T {}
class A
{
    use T;
    const X = 1;
    private $a;
    public $b;
    public function __construct(private \Rector\Tests\Transform\Rector\StaticCall\StaticCallToMethodCallRector\Source\TargetFileSystem $targetFileSystem)
    {
    }
    const Y = 2;
    /** doc */
    public function run() { return $this->targetFileSystem->dumpFile('a', 'b'); }
}
