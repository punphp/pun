<?php

namespace App {

    use Illuminate\Database\Eloquent\Model;

    class User extends Model
    {
        protected $fillable = [
            'name',
            <<<'X'
            ab
              c
            X,
        ];
    }
}
-----
<?php

namespace App {

    use Illuminate\Database\Eloquent\Model;

    #[\Illuminate\Database\Eloquent\Attributes\Fillable([
        'name',
        <<<'X'
            ab
              c
            X,
    ])]
    class User extends Model
    {
    }
}
