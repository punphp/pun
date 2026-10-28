<?php

namespace App;

use Illuminate\Foundation\Http\FormRequest;

// Rector splits the rules of the first return of `rules()` that has one and stops; its next run splits the second.
// Pun's fix rounds go on until no rule has a fix, so both returns change in one run (the known difference of rounds).
class StoreRequest extends FormRequest
{
    public function rules(): array
    {
        if ($this->isMethod('post')) {
            return ['title' => 'required|max:255'];
        }

        return ['title' => 'sometimes|max:255'];
    }
}
-----
<?php

namespace App;

use Illuminate\Foundation\Http\FormRequest;

// Rector splits the rules of the first return of `rules()` that has one and stops; its next run splits the second.
// Pun's fix rounds go on until no rule has a fix, so both returns change in one run (the known difference of rounds).
class StoreRequest extends FormRequest
{
    public function rules(): array
    {
        if ($this->isMethod('post')) {
            return ['title' => ['required', 'max:255']];
        }

        return ['title' => ['sometimes', 'max:255']];
    }
}
