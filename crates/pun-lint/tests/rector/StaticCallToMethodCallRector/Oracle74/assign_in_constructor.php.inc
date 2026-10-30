<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
	private $x;

	public function __construct(int $x)
	{
		$this->x = $x;
		// done
	}

	public function go()
	{
		return App::get($this->x);
	}
}
class B
{
    public function go() { return App::get('y'); }
}
-----
<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
	private $x;
    private \Illuminate\Foundation\Application $application;

	public function __construct(int $x, \Illuminate\Foundation\Application $application)
	{
		$this->x = $x;
		// done
        $this->application = $application;
	}

	public function go()
	{
		return $this->application->get($this->x);
	}
}
class B
{
    private \Illuminate\Foundation\Application $application;
    public function __construct(\Illuminate\Foundation\Application $application)
    {
        $this->application = $application;
    }
    public function go() { return $this->application->get('y'); }
}
