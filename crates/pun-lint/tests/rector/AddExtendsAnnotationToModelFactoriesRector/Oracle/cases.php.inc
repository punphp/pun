<?php

namespace App;

use App\Models\User;
use Illuminate\Database\Eloquent\Factories\Factory;

class A extends Factory
{
    protected $model = User::class;
}

/** Makes users. */
class B extends Factory
{
    protected $model = User::class;
}

/**
 * Makes users.
 *
 * @author me
 */
final class C extends Factory
{
    protected ?string $model = '\App\Models\Post';
}

#[\Attribute]
class D extends Factory
{
    protected $model = Unknown::class;
}

/**
 * @phpstan-extends Factory<User>
 */
class E extends Factory
{
    protected $model = User::class;
}

/**
 * @template-extends Factory<User>
 */
class F extends Factory
{
    protected $model = User::class;
}

class G extends Factory
{
    protected $model = User::class, $other = 1;
    protected $x = 1;
}

class H extends Factory
{
    protected $x;
    protected $model;
}

class I extends Factory
{
    protected $model = self::class;
}

class J extends Factory
{
    protected $model = 'App' . '\User';
}

class K
{
    protected $model = User::class;
}

class L extends A
{
    public static $model = User::class;
}

if (true) {
    class M extends Factory
    {
        protected $model = User::class;
    }
}

/* plain comment */
class N extends Factory
{
    protected $model = "App\\Models\\User";
}

/** @extends */
class O extends Factory
{
    protected $model = User::class;
}

class P extends Factory
{
    protected $MODEL = User::class;
}

/**
 * @EXTENDS Factory<User>
 */
class Q extends Factory
{
    protected $model = User::class;
}

new class extends Factory {
    protected $model = User::class;
};
-----
<?php

namespace App;

use App\Models\User;
use Illuminate\Database\Eloquent\Factories\Factory;

/**
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Models\User>
 */
class A extends Factory
{
    protected $model = User::class;
}

/**
 * Makes users.
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Models\User>
 */
class B extends Factory
{
    protected $model = User::class;
}

/**
 * Makes users.
 *
 * @author me
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Models\Post>
 */
final class C extends Factory
{
    protected ?string $model = '\App\Models\Post';
}

/**
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Unknown>
 */
#[\Attribute]
class D extends Factory
{
    protected $model = Unknown::class;
}

/**
 * @phpstan-extends Factory<User>
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Models\User>
 */
class E extends Factory
{
    protected $model = User::class;
}

/**
 * @template-extends Factory<User>
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Models\User>
 */
class F extends Factory
{
    protected $model = User::class;
}

/**
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Models\User>
 */
class G extends Factory
{
    protected $model = User::class, $other = 1;
    protected $x = 1;
}

class H extends Factory
{
    protected $x;
    protected $model;
}

/**
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\self>
 */
class I extends Factory
{
    protected $model = self::class;
}

class J extends Factory
{
    protected $model = 'App' . '\User';
}

class K
{
    protected $model = User::class;
}

/**
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Models\User>
 */
class L extends A
{
    public static $model = User::class;
}

if (true) {
    /**
     * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Models\User>
     */
    class M extends Factory
    {
        protected $model = User::class;
    }
}

/* plain comment */
/**
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Models\User>
 */
class N extends Factory
{
    protected $model = "App\\Models\\User";
}

/** @extends */
class O extends Factory
{
    protected $model = User::class;
}

/**
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Models\User>
 */
class P extends Factory
{
    protected $MODEL = User::class;
}

/**
 * @EXTENDS Factory<User>
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Models\User>
 */
class Q extends Factory
{
    protected $model = User::class;
}

new class extends Factory {
    protected $model = User::class;
};
