<?php

use Illuminate\Database\Eloquent\Model;

#[Foo] class A extends Model {
    protected $fillable = ['a'];
    public $x;
}

    #[Foo, Bar(1)]
    final class B extends Model {
        protected $fillable = ['a'];
        public $x;
    }

/** doc */ final class C extends Model {
    protected $fillable = ['a'];
    public $x;
}

final
class D extends Model {
    protected $fillable = ['a'];
    public $x;
}
-----
<?php

use Illuminate\Database\Eloquent\Model;

#[Foo]
#[\Illuminate\Database\Eloquent\Attributes\Fillable(['a'])] class A extends Model {
    public $x;
}

    #[Foo, Bar(1)]
    #[\Illuminate\Database\Eloquent\Attributes\Fillable(['a'])]
    final class B extends Model {
        public $x;
    }

/** doc */ #[\Illuminate\Database\Eloquent\Attributes\Fillable(['a'])]
final class C extends Model {
    public $x;
}

#[\Illuminate\Database\Eloquent\Attributes\Fillable(['a'])]
final
class D extends Model {
    public $x;
}
