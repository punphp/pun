<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
	public function go()
	{
		return App::get(1);
	}
}
-----
<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
	public function __construct(private \Illuminate\Foundation\Application $application)
    {
    }
    public function go()
	{
		return $this->application->get(1);
	}
}
