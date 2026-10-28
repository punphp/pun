<?php

namespace App;

use Illuminate\Foundation\Http\FormRequest;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Validator;

class StoreRequest extends FormRequest
{
    public function rules(): array
    {
        if ($this->isMethod('post')) {
            return $this->other();
        }

        return ['title' => 'required|max:255', 'body' => 'x'];
    }

    public function messages(): array
    {
        return ['title' => 'a|b'];
    }
}

class ChildRequest extends StoreRequest
{
    public function rules(): array
    {
        return array(
            'a' => "required|in:it's|\$x",
            'b' => 'one',
            'c' => ['x|y'],
            'd' => '',
            'e' => 'a||b',
            'f' => 'a\'b|c\\d',
            'tags.*' => <<<EOT
                x|y
                  z|\x41|q\"t|\\\"u
                EOT,
            'now' => <<<'EOT'
            a|\x41
            EOT,
            'int' => <<<EOT
              a|{$this->x}|b
              c
              EOT,
            'flat' => <<<EOT
            a|b
            EOT,
            'g' => "a|{$this->x}|",
            'h' => "\t|\n",
        );
    }
}

function run(Request $request, ?Request $maybe, $unknown, array $data)
{
    $request->validate(['a' => 'required|string', 0 => 'x|y', 'nested' => ['z' => 'a|b']]);
    $request->validate(rules: ['a' => 'b|c']);
    $request->validate($data);
    $maybe->validate(['a' => 'b|c']);
    $unknown->validate(['a' => 'b|c']);
    Validator::make($data, ['a' => 'b|c']);
    Validator::make($data, ['a' => 'b|c'], ['msg' => 'x|y']);
    Validator::make(['a' => 'b|c']);
    \Validator::make($data, ['a' => 'b|c']);
}
-----
<?php

namespace App;

use Illuminate\Foundation\Http\FormRequest;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Validator;

class StoreRequest extends FormRequest
{
    public function rules(): array
    {
        if ($this->isMethod('post')) {
            return $this->other();
        }

        return ['title' => ['required', 'max:255'], 'body' => ['x']];
    }

    public function messages(): array
    {
        return ['title' => 'a|b'];
    }
}

class ChildRequest extends StoreRequest
{
    public function rules(): array
    {
        return array(
            'a' => ['required', 'in:it\'s', '$x'],
            'b' => ['one'],
            'c' => ['x|y'],
            'd' => [''],
            'e' => ['a', '', 'b'],
            'f' => ['a\'b', 'c\d'],
            'tags.*' => ['x', 'y
  z', 'A', 'q\"t', '\\\\"u'],
            'now' => ['a', '\x41'],
            'int' => ["a", "{$this->x}", "b\nc"],
            'flat' => ['a', 'b'],
            'g' => ["a", "{$this->x}"],
            'h' => ['	', '
'],
        );
    }
}

function run(Request $request, ?Request $maybe, $unknown, array $data)
{
    $request->validate(['a' => ['required', 'string'], 0 => ['x', 'y'], 'nested' => ['z' => 'a|b']]);
    $request->validate(rules: ['a' => ['b', 'c']]);
    $request->validate($data);
    $maybe->validate(['a' => ['b', 'c']]);
    $unknown->validate(['a' => 'b|c']);
    Validator::make($data, ['a' => ['b', 'c']]);
    Validator::make($data, ['a' => 'b|c'], ['msg' => 'x|y']);
    Validator::make(['a' => 'b|c']);
    \Validator::make($data, ['a' => 'b|c']);
}
