<?php

namespace Illuminate\Foundation;

use Illuminate\Support\HtmlString;
use Illuminate\Support\Str;

class Mix
{
    /**
     * Get the path to a versioned Mix file.
     *
     * @param  string  $path
     * @param  string  $manifestDirectory
     * @return \Illuminate\Support\HtmlString|string
     *
     * @throws \Illuminate\Foundation\MixManifestNotFoundException|\Illuminate\Foundation\MixFileNotFoundException
     */
    public function __invoke($path, $manifestDirectory = '')
    {}
}
