<?php

namespace App\Http;

use Illuminate\Support\Facades\{Redirect, Route};
use Illuminate\Support as S;

class C
{
    public function a()
    {
        return [Redirect::back(), S\Facades\Redirect::back(), Facades\Redirect::back()];
    }
}
-----
<?php

namespace App\Http;

use Illuminate\Support\Facades\{Redirect, Route};
use Illuminate\Support as S;

class C
{
    public function a()
    {
        return [back(), back(), Facades\Redirect::back()];
    }
}
