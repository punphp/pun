<?php

namespace App;

use Illuminate\Foundation\Http\FormRequest;
use Illuminate\Http\Request;

abstract class BaseRequest extends FormRequest
{
    const LIMIT = 5;

    public $max = 9;

    abstract public function rules(): array;
}

class MoreRequest extends BaseRequest
{
    public function rules(): array
    {
        return [
            'a' => "required|max:$this->max",
            'b' => <<<EOT
required|max:5
EOT,
            'c' => <<<EOT
                required|max:$this->max|min:1
                EOT,
            'd' => <<<EOT
                required|max:5

                EOT,
        ];
    }
}

function run(Request $request)
{
    $method = 'validate';
    $request->$method(['a' => 'b|c']);
}
-----
<?php

namespace App;

use Illuminate\Foundation\Http\FormRequest;
use Illuminate\Http\Request;

abstract class BaseRequest extends FormRequest
{
    const LIMIT = 5;

    public $max = 9;

    abstract public function rules(): array;
}

class MoreRequest extends BaseRequest
{
    public function rules(): array
    {
        return [
            'a' => ["required", "max:{$this->max}"],
            'b' => ['required', 'max:5'],
            'c' => ["required", "max:{$this->max}", "min:1"],
            'd' => ['required', 'max:5
'],
        ];
    }
}

function run(Request $request)
{
    $method = 'validate';
    $request->$method(['a' => 'b|c']);
}
