<?php

namespace Illuminate\Foundation;

class Precognition
{
    /**
     * Get the "after" validation hook that can be used for precognition requests.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return \Closure
     */
    public static function afterValidationHook($request)
    {}
}
