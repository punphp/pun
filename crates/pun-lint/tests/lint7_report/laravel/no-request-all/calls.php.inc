<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Request as RequestFacade;

function store(Request $request)
{
    $request->all();
    $request->ALL();
    $REQUEST->all();
    $req->all();
    request()->all();
    request('x')->all();
    \request()->all();
    Request::all();
    RequestFacade::all();
    \Illuminate\Http\Request::all();
    Other::all();
    $request?->all();
    $request->only(['a']);
    static::all();
    $class::all();
    $request->all(...);
    Request::all(...);
    $f = function () use ($request) {
        return $request->all();
    };
    $g = fn () => $request->all();
    if (true) {
        foreach ([] as $x) {
            $request->all();
        }
    }
    return $request->all()['x'] ?? $request->{'all'}();
}

class C
{
    public function m(Request $request)
    {
        $request->all();
        new class {
            public function n($request)
            {
                return $request->all();
            }
        };
    }

}
-----
10:5: Avoid using `$request->all()` or `Request::all()`.
11:5: Avoid using `$request->all()` or `Request::all()`.
12:5: Avoid using `$request->all()` or `Request::all()`.
14:5: Avoid using `$request->all()` or `Request::all()`.
16:5: Avoid using `$request->all()` or `Request::all()`.
17:5: Avoid using `$request->all()` or `Request::all()`.
18:5: Avoid using `$request->all()` or `Request::all()`.
19:5: Avoid using `$request->all()` or `Request::all()`.
28:16: Avoid using `$request->all()` or `Request::all()`.
33:13: Avoid using `$request->all()` or `Request::all()`.
36:12: Avoid using `$request->all()` or `Request::all()`.
43:9: Avoid using `$request->all()` or `Request::all()`.
47:24: Avoid using `$request->all()` or `Request::all()`.
