<?php

namespace App;

use Illuminate\Contracts\Foundation\Application as Contract;
use Illuminate\Foundation\Application;
use Illuminate\Support\Facades\App;

class Provider
{
    public function __construct(private Application $app, private Contract $contract)
    {
    }

    public function run(?Application $maybe, $unknown, string $env)
    {
        $a = $this->app->environment() <> 'local';
        $b = ($this->app->environment()) == ('production');
        $c = 'local' === $this->contract->environment();
        $d = $this->app->environment() === $this->contract->environment();
        $e = in_array(haystack: ['local', 'testing'], needle: $this->app->environment());
        $f = App::environment() !== 'production';
        $g = $maybe->environment() === 'local';
        $h = $unknown->environment() === 'local';
        $i = $this->app->environment('local') === 'local';
        $j = $this->app->environment() === $env;
        $k = $this->app->ENVIRONMENT() != "local";
        $l = $this->app->environment() > 'local';
        $m = in_array($this->app->environment());
        $n = in_array($env, [$this->app->environment()]);
        $o = $this->app?->environment() === 'local';
    }
}
-----
<?php

namespace App;

use Illuminate\Contracts\Foundation\Application as Contract;
use Illuminate\Foundation\Application;
use Illuminate\Support\Facades\App;

class Provider
{
    public function __construct(private Application $app, private Contract $contract)
    {
    }

    public function run(?Application $maybe, $unknown, string $env)
    {
        $a = !$this->app->isLocal();
        $b = $this->app->isProduction();
        $c = $this->contract->environment('local');
        $d = $this->app->environment($this->contract->environment());
        $e = $this->app->environment(['local', 'testing']);
        $f = !App::isProduction();
        $g = $maybe->isLocal();
        $h = $unknown->environment() === 'local';
        $i = $this->app->environment('local') === 'local';
        $j = $this->app->environment($env);
        $k = !$this->app->isLocal();
        $l = $this->app->environment() > 'local';
        $m = in_array($this->app->environment());
        $n = in_array($env, [$this->app->environment()]);
        $o = $this->app?->environment() === 'local';
    }
}
