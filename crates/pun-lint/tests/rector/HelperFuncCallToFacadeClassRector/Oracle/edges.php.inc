<?php
namespace {
app('service');
resolve(abstract: Foo::class, parameters: ['id' => 2]);
\APP(null);
\ReSoLvE(...$args);
app();
resolve();
other('service');
$app('service');
Vendor\app('service');
namespace\app('service');
${'app'}('service');
$$app('service');
(app)('service');
($object->app)('service');
($object?->resolve)('service');
(Example::$app)('service');
($object->$method)('service');
($object?->$method)('service');
(Example::${$method})('service');
(Example::app)('service');
app()('service');
$app()('service');
($object->app())('service');
}
namespace Example {
app('service');
namespace\app('service');
\app('service');
}
namespace Imported {
use function Vendor\app;
use function app as application;
app('service');
application('service');
}
-----
<?php
namespace {
\Illuminate\Support\Facades\App::make('service');
\Illuminate\Support\Facades\App::make(abstract: Foo::class, parameters: ['id' => 2]);
\Illuminate\Support\Facades\App::make(null);
\Illuminate\Support\Facades\App::make(...$args);
app();
resolve();
other('service');
\Illuminate\Support\Facades\App::make('service');
Vendor\app('service');
\Illuminate\Support\Facades\App::make('service');
${'app'}('service');
$$app('service');
\Illuminate\Support\Facades\App::make('service');
\Illuminate\Support\Facades\App::make('service');
\Illuminate\Support\Facades\App::make('service');
\Illuminate\Support\Facades\App::make('service');
($object->$method)('service');
($object?->$method)('service');
(Example::${$method})('service');
(Example::app)('service');
\Illuminate\Support\Facades\App::make('service');
$app()('service');
($object->app())('service');
}
namespace Example {
\Illuminate\Support\Facades\App::make('service');
namespace\app('service');
\Illuminate\Support\Facades\App::make('service');
}
namespace Imported {
use function Vendor\app;
use function app as application;
app('service');
\Illuminate\Support\Facades\App::make('service');
}
