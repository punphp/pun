<?php

namespace App;

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Request as RequestFacade;

class FormRequest extends Request {}

interface HasStore
{
    public function store();
}

abstract class Base
{
    abstract public function make();
    private function secret() {}
    protected function shared() {}
}

class Controller extends Base implements HasStore
{
    public function a()
    {
        return Request::validate(['a' => 'required']);
    }

    public function b(int $id)
    {
        return Request::all();
    }

    public function c(
        int $id,
        string $name
    ) {
        return request()->all();
    }

    public function d(int $id,)
    {
        return request('a', 'b');
    }

    public function e(FormRequest $req)
    {
        return [request('x'), request(), Request::input('y')];
    }

    public function f(?Request $r)
    {
        return request();
    }

    public function h()
    {
        return fn () => Request::get('x');
    }

    public function store()
    {
        return request('x');
    }

    public function make()
    {
        return request('x');
    }

    public function secret()
    {
        return request('x');
    }

    public function shared()
    {
        return request('x');
    }

    public function i()
    {
        return RequestFacade::input('x');
    }

    public function j()
    {
        return \Request::input('x');
    }

    public function k()
    {
        return \request('x');
    }

    public static function l()
    {
        return request('x');
    }

    public function n()
    {
        return Request::validate(
            ['a' => 'required'],
            ['a.required' => 'msg'],
        );
    }

    public function o()
    {
        return FormRequest::capture();
    }

    public function u()
    {
        return Request::input(request('x'));
    }

    public function v()
    {
        $class = Request::class;
        return [$class::input('a'), Request::{'all'}(), new class(request('b')) {}];
    }

    public function w(self $s, Request|FormRequest $req)
    {
        return Request::all();
    }

    public function x(int $a = 1)
    {
        return Request::all();
    }

    public function __construct()
    {
        request('x');
    }

    abstract function q();

}

trait T
{
    public function t()
    {
        return request('x');
    }
}

-----
<?php

namespace App;

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Request as RequestFacade;

class FormRequest extends Request {}

interface HasStore
{
    public function store();
}

abstract class Base
{
    abstract public function make();
    private function secret() {}
    protected function shared() {}
}

class Controller extends Base implements HasStore
{
    public function a(\Illuminate\Http\Request $request)
    {
        return $request->validate(['a' => 'required']);
    }

    public function b(int $id, \Illuminate\Http\Request $request)
    {
        return $request->all();
    }

    public function c(
        int $id,
        string $name,
        \Illuminate\Http\Request $request
    ) {
        return $request->all();
    }

    public function d(int $id, \Illuminate\Http\Request $request,)
    {
        return $request->input('a', 'b');
    }

    public function e(FormRequest $req)
    {
        return [$req->input('x'), $req, $req->input('y')];
    }

    public function f(?Request $r)
    {
        return $r;
    }

    public function h(\Illuminate\Http\Request $request)
    {
        return fn () => $request->get('x');
    }

    public function store()
    {
        return request('x');
    }

    public function make()
    {
        return request('x');
    }

    public function secret(\Illuminate\Http\Request $request)
    {
        return $request->input('x');
    }

    public function shared()
    {
        return request('x');
    }

    public function i()
    {
        return RequestFacade::input('x');
    }

    public function j(\Illuminate\Http\Request $request)
    {
        return $request->input('x');
    }

    public function k(\Illuminate\Http\Request $request)
    {
        return $request->input('x');
    }

    public static function l(\Illuminate\Http\Request $request)
    {
        return $request->input('x');
    }

    public function n(\Illuminate\Http\Request $request)
    {
        return $request->validate(['a' => 'required'], ['a.required' => 'msg']);
    }

    public function o(\Illuminate\Http\Request $request)
    {
        return $request->capture();
    }

    public function u(\Illuminate\Http\Request $request)
    {
        return $request->input($request->input('x'));
    }

    public function v(\Illuminate\Http\Request $request)
    {
        $class = Request::class;
        return [$class::input('a'), Request::{'all'}(), new class($request->input('b')) {}];
    }

    public function w(self $s, Request|FormRequest $req)
    {
        return $req->all();
    }

    public function x(int $a = 1, \Illuminate\Http\Request $request)
    {
        return $request->all();
    }

    public function __construct(\Illuminate\Http\Request $request)
    {
        $request->input('x');
    }

    abstract function q();

}

trait T
{
    public function t()
    {
        return request('x');
    }
}

