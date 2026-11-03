<?php

namespace Illuminate\View;

use ErrorException;
use Illuminate\Container\Container;
use Illuminate\Support\Reflector;

class ViewException extends ErrorException
{
    /**
     * Report the exception.
     *
     * @return bool|null
     */
    public function report()
    {}

    /**
     * Render the exception into an HTTP response.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return \Illuminate\Http\Response|null
     */
    public function render($request)
    {}
}
