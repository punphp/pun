<?php

use Illuminate\Database\Eloquent\Model;

function f() {
    if (true) {
        $a = new #[Foo] readonly class(1, 2) extends Model implements \Countable {
            protected $fillable = ['a'];
        };
        $b = new #[Foo] class extends Model {
            protected $fillable = [
                'a',
            ];
            public $x;
        };
    }
}
-----
<?php

use Illuminate\Database\Eloquent\Model;

function f() {
    if (true) {
        $a = new #[Foo]
        #[\Illuminate\Database\Eloquent\Attributes\Fillable(['a'])]
        readonly class(1, 2) extends Model implements \Countable
        {
        };
        $b = new #[Foo] #[\Illuminate\Database\Eloquent\Attributes\Fillable([
            'a',
        ])] class extends Model {
            public $x;
        };
    }
}
