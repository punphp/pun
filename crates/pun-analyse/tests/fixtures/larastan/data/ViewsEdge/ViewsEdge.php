<?php

namespace App\PunViews;

use Illuminate\Contracts\View\Factory;
use Illuminate\Support\Facades\Lang;
use Illuminate\Support\Facades\View;

class ViewsEdge
{
    public function run(string $name, callable $fn, string $class, Factory $factory): void
    {
        view('punedge.translations');
        __('punedge.int.6');
        __('punedge.missing');
        view($name);
        view('punpkg::some/view');
        $fn('punedge.dynamic');
        $class::make('punedge.dynamic');
        View::share('punedge.shared', 'x');
        Lang::has('punedge.has');
        collect(['k' => 'v'])->get('k');
        $factory->make($name);
    }
}
