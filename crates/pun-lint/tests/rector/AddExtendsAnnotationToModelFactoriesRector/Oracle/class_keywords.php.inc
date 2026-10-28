<?php

namespace App\Factories;

use Illuminate\Database\Eloquent\Factories\Factory;

class SelfFactory extends Factory
{
    protected $model = self::class;
}
-----
<?php

namespace App\Factories;

use Illuminate\Database\Eloquent\Factories\Factory;

/**
 * @extends \Illuminate\Database\Eloquent\Factories\Factory<\self>
 */
class SelfFactory extends Factory
{
    protected $model = self::class;
}
