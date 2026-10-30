<?php

namespace Database\Factories;

use App\Models\User;
use Illuminate\Database\Eloquent\Factories\Factory;

/**
 */
class AFactory extends Factory
{
    protected $model = User::class;
}

/** */
class BFactory extends Factory
{
    protected $model = User::class;
}

/**
 *
 */
class CFactory extends Factory
{
    protected $model = User::class;
}

/***/
class DFactory extends Factory
{
    protected $model = User::class;
}
-----
<?php

namespace Database\Factories;

use App\Models\User;
use Illuminate\Database\Eloquent\Factories\Factory;

/**
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Models\User>
 */
class AFactory extends Factory
{
    protected $model = User::class;
}

/**
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Models\User>
 */
class BFactory extends Factory
{
    protected $model = User::class;
}

/**
 *
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Models\User>
 */
class CFactory extends Factory
{
    protected $model = User::class;
}

/***/
/**
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\App\Models\User>
 */
class DFactory extends Factory
{
    protected $model = User::class;
}
