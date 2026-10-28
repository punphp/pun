<?php

namespace App;

use Illuminate\Contracts\Container\Container;
use Illuminate\Contracts\Session\Session;

class Base
{
    const SERVICE = 'encrypter';
}

class Services extends Base
{
    const SERVICE = 'session';

    public function run(Container $app, $x)
    {
        $app->make(self::SERVICE);
        $app->make(static::SERVICE);
        $app->make(parent::SERVICE);
        $app->make($x::SERVICE);
        $app->make(Session::{'class'});
        $app->make(Session::class);
    }
}
-----
<?php

namespace App;

use Illuminate\Contracts\Container\Container;
use Illuminate\Contracts\Session\Session;

class Base
{
    const SERVICE = 'encrypter';
}

class Services extends Base
{
    const SERVICE = 'session';

    public function run(Container $app, $x)
    {
        $app->make(self::SERVICE);
        $app->make(static::SERVICE);
        $app->make(parent::SERVICE);
        $app->make($x::SERVICE);
        $app->make('store');
        $app->make('store');
    }
}
